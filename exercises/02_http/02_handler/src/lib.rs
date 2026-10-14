use anyhow::{Context, bail};
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{IntoResponse, Method, Request, Response, StatusCode};
use spin_sdk::http_service;
use std::iter::Peekable;
use std::str::FromStr;

// curl --request POST --data '10 / 2' http://localhost:3000/

#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.method() != Method::POST {
        return text(StatusCode::METHOD_NOT_ALLOWED, "only POST is supported");
    }

    let body = req.into_body().bytes().await?;
    let input = str::from_utf8(&body)?;

    match evaluate(input) {
        Ok(result) => text(StatusCode::OK, result.to_string()),
        Err(err) => text(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

fn text(status: StatusCode, body: impl Into<String>) -> anyhow::Result<Response<String>> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "text/plain")
        .body(body.into())?)
}

fn evaluate(input: &str) -> anyhow::Result<f64> {
    let tokens = tokenize(input)?;
    let mut evaluator = Evaluator {
        tokens: tokens.into_iter().peekable(),
    };
    let result = evaluator.expression()?;
    if let Some(token) = evaluator.tokens.next() {
        bail!("unexpected token {token:?}");
    }
    Ok(result)
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
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
    // expression := term (("+" | "-") term)*
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

    // term := factor (("*" | "/") factor)*
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

    // factor := number | "-" factor | "(" expression ")"
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
