#[warn(dead_code)]
use std::str::Chars;

pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
}

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
}

impl Cursor<'_> {
    pub fn consume_next_token(&mut self) -> Token {
        Token {
            token_type: TokenType::Eof,
            value: None,
        }
    }
}

fn tokenize(code: &str) -> impl Iterator<Item=Token> + '_ {
    let mut cursor = Cursor::new(code);
    std::iter::from_fn(move || {
        let token = cursor.consume_next_token();
        if token.token_type == TokenType::Eof { Some(token) } else { None }
    })
}

struct Token {
    token_type: TokenType,
    // fixme try with nonstatic lifetime
    value: Option<&'static str>
}

#[derive(PartialEq)]
enum TokenType {
    // ;
    Semi,
    // " "
    Whitespace,
    // "let"
    Let,
    // "return"
    Return,
    // starts with alpha can contain digits i.e. xyz / xyz123
    Variable,
    // digits i.e. 123
    Value,
    // end of str
    Eof,
}

#[test]
fn first_tokenozing_test() {
    let code = "return 1;";
    let cursor = Cursor::new(code);
    let tokenized_code = [TokenType::Return, TokenType::Value, TokenType::Semi];
    assert_eq!(cursor::tokenize(code), tokenized_code)
}
