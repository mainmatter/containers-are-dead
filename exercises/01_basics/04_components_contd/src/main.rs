use std::env;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

// This component should:
// 1. read a single string argument from the command line
// 2. parse that input into a set of tokens as youve done before (reuse your code!)
// 2. Evaluate these tokens according to the operator precedence rules (parentheses first, multiple/divide next, the plus/minus)
// 3. Print out the result to stdout
// If you get stuck, ask a neighbor or grab a trainer!
pub fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 1);

    let result = Evaluator::new(&args[0]).eval();

    println!("{result}");
}

struct Evaluator<'a> {
    parser: Peekable<Parser<'a>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            parser: Parser::new(input).peekable(),
        }
    }

    pub fn eval(mut self) -> f64 {
        self.eval_expression()
    }

    fn eval_expression(&mut self) -> f64 {
        let mut result = self.eval_term();

        while let Some(token) = self.parser.peek() {
            match token {
                Token::Plus => {
                    self.parser.next();
                    result += self.eval_term();
                }
                Token::Minus => {
                    self.parser.next();
                    result -= self.eval_term();
                }
                _ => break,
            }
        }

        result
    }

    fn eval_term(&mut self) -> f64 {
        let mut result = self.eval_factor();

        while let Some(token) = self.parser.peek() {
            match token {
                Token::Multiply => {
                    self.parser.next();
                    result *= self.eval_factor();
                }
                Token::Divide => {
                    self.parser.next();
                    result /= self.eval_factor();
                }
                _ => break,
            }
        }

        result
    }

    fn eval_factor(&mut self) -> f64 {
        match self.parser.next().expect("unexpected EOF") {
            Token::Number(n) => n,
            Token::LeftParen => {
                let result = self.eval_expression();
                match self.parser.next().expect("unexpected EOF") {
                    Token::RightParen => result,
                    _ => panic!("mismatched parentheses"),
                }
            }
            Token::Minus => -self.eval_factor(),
            t => panic!("mismatched token {t:?}"),
        }
    }
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

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            chars: raw.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.next()?;

        let token = match ch {
            ' ' | '\t' => {
                while self.chars.next_if(|c| *c == ' ' || *c == '\t').is_some() {}
                return self.next();
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            c if c.is_numeric() => {
                let mut str = c.to_string();
                while let Some(ch) = self.chars.next_if(|c| c.is_numeric() || *c == '.') {
                    str.push(ch);
                }
                Token::Number(f64::from_str(&str).unwrap())
            }
            _ => panic!("unexpected character: {ch}"),
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(Evaluator::new("2 + 3").eval(), 5.0);
        assert_eq!(Evaluator::new("10 - 4").eval(), 6.0);
        assert_eq!(Evaluator::new("3 * 4").eval(), 12.0);
        assert_eq!(Evaluator::new("15 / 3").eval(), 5.0);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(Evaluator::new("2 + 3 * 4").eval(), 14.0);
        assert_eq!(Evaluator::new("(2 + 3) * 4").eval(), 20.0);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Evaluator::new("-5 + 3").eval(), -2.0);
        assert_eq!(Evaluator::new("10 + -5").eval(), 5.0);
    }
}
