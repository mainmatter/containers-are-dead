// This module has *several* issues:
// 1. First we're missing a parser implementation!
//   This parser should accept input as a `&str` and parse the tokens into a `Vec`. See the `Token` enum below and the test cases for details.
// 2. The function declaration below is NOT valid! `&str` and `Vec<Token>` cannot be passed across an FFI boundary! You will need to fix this as well!
// If you get stuck, ask a neighbor or grab a trainer!

use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::slice;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

#[unsafe(no_mangle)]
//unsafe extern "C" fn parse_expression<'a>(input: &str) -> Vec<Token> {
unsafe extern "C" fn parse_expression<'a>(input: *const u8, len: usize,  out_ptr: *mut c_void, out_len: usize) -> usize {
    let slice = unsafe { std::slice::from_raw_parts(input, len) };
    let string = std::str::from_utf8(slice).unwrap();


    // let tks : Vec<&str> = string.split_whitespace().collect();

    // let tks : Vec<Token> = tks.iter().map(|t| {
    //     match *t {
    //         "+" => Token::Plus,
    //         "-" => Token::Minus,
    //         "*" => Token::Multiply,
    //         "/" => Token::Divide,
    //         "(" => Token::LeftParen,
    //         ")" => Token::RightParen,
    //         _ => Token::Number(t.parse::<f64>().unwrap_or_default()),
    //     }
    // }).collect();
    // let out_buf =
    //     unsafe { slice::from_raw_parts_mut(out_ptr.cast::<MaybeUninit<Token>>(), out_len) };
    // let mut tokens_emitted = 0;

    // for (token, slot) in tks.iter().zip(out_buf.iter_mut()) {
    //     slot.write(token.clone());
    //     tokens_emitted += 1;
    // }

    let out_buf =
        unsafe { slice::from_raw_parts_mut(out_ptr.cast::<MaybeUninit<Token>>(), out_len) };
    let mut tokens_emitted = 0;

    for (token, slot) in Parser::new(string).zip(out_buf.iter_mut()) {
        slot.write(token);
        tokens_emitted += 1;
    }

    tokens_emitted
}

// The token that our parse should recognize
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Numbers: 1, or 4.5, or 100000000. Must fit within an f64.
    Number(f64),
    /// The `+` symbol
    Plus,
    /// The `-` symbol
    Minus,
    /// The `*` symbol
    Multiply,
    /// The `/` divide symbol
    Divide,
    /// The `(` symbol
    LeftParen,
    /// The `)` symbol
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
