use std::path::Prefix;
use std::str::Chars;
use expect_test::{expect, Expect};

pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
}

pub const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(content: &'a str) -> Self {
        Cursor {
            len_remaining: content.len(),
            chars: content.chars(),
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn next(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }
}

impl<'a> Cursor<'a> {
    fn identifier_length(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }
}

impl Cursor<'_> {
    pub fn consume_next_token(&mut self) -> Token {
        let first_char = match self.peek() {
            Some(fc) => fc,
            None => return Token::new(TokenType::Eof, 0)
        };
        let token_type = match first_char {
            ';' => TokenType::Semi,
            ' ' => TokenType::Whitespace,
            '0'..='9' => {
                self.consume_until(move |x| { x.is_digit(10) });
                TokenType::IntLiteral
            }
            c if c.is_alphabetic() => {
                self.consume_until(move |x| { x.is_alphanumeric() });
                TokenType::Identifier
            }
            _ => TokenType::Eof
        };
        Token::new(token_type, 0)
    }

    // returns content until next whitespace
    fn content_until_whitespace(&self) -> &str {
        ""
    }

    // moves cursor to the next whitespace
    fn consume_until_whitespace(&mut self) {
        while self.next() != ' ' && !self.chars.as_str().is_empty() {
            self.peek();
        }
    }

    fn consume_until(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.next()) && !self.chars.as_str().is_empty() {
            self.peek();
        }
    }

    fn len_until(&mut self, mut predicate: impl FnMut(char) -> bool) -> usize {
        let mut length: usize = 0;
        while predicate(self.next()) && !self.chars.as_str().is_empty() {
            self.peek();
            length += 1;
        }
        length
    }
}

fn tokenize(code: &str) -> impl Iterator<Item=Token> + '_ {
    let mut cursor = Cursor::new(code);
    std::iter::from_fn(move || {
        let token = cursor.consume_next_token();
        if token.token_type != TokenType::Eof { Some(token) } else { None }
    })
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    // identifier length
    pub value_len: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value_len: usize) -> Self {
        Token { token_type, value_len }
    }
}

#[derive(PartialEq, Debug)]
enum TokenType {
    // ;
    Semi,
    // " "
    Whitespace,
    // "return"
    Identifier,
    // digits i.e. 123
    IntLiteral,
    // end of str
    Eof,
}


fn test_tokenizing(code: &str, expect: Expect) {
    let actual: String = tokenize(code)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual);
}

#[test]
fn first_tokenizing_test() {
    let code = "return 1;";
    let expected = expect![[r#"
            Token { token_type: Identifier, value_len: 0 }
            Token { token_type: Whitespace, value_len: 0 }
            Token { token_type: IntLiteral, value_len: 0 }
            Token { token_type: Semi, value_len: 0 }
            "#
        ]];
    test_tokenizing(code, expected);
}

