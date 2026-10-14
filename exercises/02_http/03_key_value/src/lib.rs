use anyhow::{Context, bail};
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{IntoResponse, Method, Request, Response, StatusCode};
use spin_sdk::http_service;
use spin_sdk::key_value::Store;
use std::iter::Peekable;
use std::str::FromStr;

// curl --request POST --data 'a = 10' http://localhost:3000/
// curl --request POST --data 'a * 2' http://localhost:3000/

#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.method() != Method::POST {
        return text(StatusCode::METHOD_NOT_ALLOWED, "only POST is supported");
    }

    let body = req.into_body().bytes().await?;
    let input = str::from_utf8(&body)?;

    let store = Store::open_default().await?;

    match evaluate(input, &store).await {
        Ok(Some(result)) => text(StatusCode::OK, result.to_string()),
        Ok(None) => text(StatusCode::OK, ""),
        Err(err) => text(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

fn text(status: StatusCode, body: impl Into<String>) -> anyhow::Result<Response<String>> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "text/plain")
        .body(body.into())?)
}

/// Evaluates an expression, or stores the result of an assignment and returns `None`.
async fn evaluate(input: &str, store: &Store) -> anyhow::Result<Option<f64>> {
    let mut tokens = tokenize(input)?;

    // `<ident> = <expression>`
    let assign_to = match tokens.as_slice() {
        [Token::Identifier(name), Token::Assign, ..] => Some(name.clone()),
        _ => None,
    };
    if assign_to.is_some() {
        tokens.drain(..2);
    }

    // identifiers are looked up before evaluating, so the evaluator stays synchronous
    for token in &mut tokens {
        if let Token::Identifier(name) = token {
            let value: f64 = store
                .get_json(name.as_str())
                .await?
                .with_context(|| format!("unknown variable `{name}`"))?;
            *token = Token::Number(value);
        }
    }

    let mut evaluator = Evaluator {
        tokens: tokens.into_iter().peekable(),
    };
    let result = evaluator.expression()?;
    if let Some(token) = evaluator.tokens.next() {
        bail!("unexpected token {token:?}");
    }

    match assign_to {
        Some(name) => {
            store.set_json(&name, &result).await?;
            Ok(None)
        }
        None => Ok(Some(result)),
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> anyhow::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        let token = match ch {
            ' ' | '\t' | '\n' => continue,
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            c if c.is_ascii_digit() => {
                let mut str = c.to_string();
                while let Some(ch) = chars.next_if(|c| c.is_ascii_digit() || *c == '.') {
                    str.push(ch);
                }
                Token::Number(f64::from_str(&str)?)
            }
            c if c.is_ascii_alphabetic() => {
                let mut str = c.to_string();
                while let Some(ch) = chars.next_if(|c| c.is_ascii_alphanumeric() || *c == '_') {
                    str.push(ch);
                }
                Token::Identifier(str)
            }
            _ => bail!("unexpected character `{ch}`"),
        };
        tokens.push(token);
    }

    Ok(tokens)
}

struct Evaluator {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Evaluator {
    fn expression(&mut self) -> anyhow::Result<f64> {
        let mut result = self.term()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus => {
                    self.tokens.next();
                    result += self.term()?;
                }
                Token::Minus => {
                    self.tokens.next();
                    result -= self.term()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn term(&mut self) -> anyhow::Result<f64> {
        let mut result = self.factor()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Multiply => {
                    self.tokens.next();
                    result *= self.factor()?;
                }
                Token::Divide => {
                    self.tokens.next();
                    result /= self.factor()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn factor(&mut self) -> anyhow::Result<f64> {
        match self.tokens.next().context("unexpected end of input")? {
            Token::Number(n) => Ok(n),
            Token::Minus => Ok(-self.factor()?),
            Token::LeftParen => {
                let result = self.expression()?;
                match self.tokens.next() {
                    Some(Token::RightParen) => Ok(result),
                    _ => bail!("missing closing parenthesis"),
                }
            }
            token => bail!("unexpected token {token:?}"),
        }
    }
}
