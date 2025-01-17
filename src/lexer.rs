use core::fmt;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(Vec<u8>),
    Operator(Vec<u8>),
    OpenParenthesis,
    ClosedParenthesis,
}

trait CheckableChar {
    fn is_ascii_operator(&self) -> bool;
}

impl CheckableChar for u8 {
    fn is_ascii_operator(&self) -> bool {
        *self == b'+' || *self == b'-' || *self == b'*' || *self == b'/'
    }
}

#[derive(Debug)]
pub struct LexerError {}

impl LexerError {
    fn new() -> LexerError {
        LexerError {}
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexer error encountered")
    }
}
impl Error for LexerError {}

pub trait LexerString {
    fn get_next_char(&self) -> u8;
    fn get_current_char(&self) -> u8;
    fn shift_chars(&mut self);
    fn consume_char_type(&mut self, char_type: fn(&u8) -> bool) -> Vec<u8>;
    fn skip_whitespace(&mut self) -> bool;
    fn eof(&self) -> bool;
}

pub struct VecLexerString {
    string: Vec<u8>,
    current_char: usize,
    next_char: usize,
}

impl LexerString for VecLexerString {
    fn get_next_char(&self) -> u8 {
        if self.next_char >= self.string.len() {
            return b'\0';
        }
        self.string[self.next_char]
    }

    fn get_current_char(&self) -> u8 {
        if self.current_char >= self.string.len() {
            return b'\0';
        }
        self.string[self.current_char]
    }

    fn shift_chars(&mut self) {
        self.current_char += 1;
        self.next_char += 1;
    }

    fn consume_char_type(&mut self, char_type: fn(&u8) -> bool) -> Vec<u8> {
        let mut content: Vec<u8> = vec![];
        while char_type(&self.get_current_char()) {
            content.push(self.get_current_char());
            self.shift_chars();
        }
        content
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut skipped = false;
        while self.get_current_char().is_ascii_whitespace() {
            skipped = true;
            self.shift_chars();
        }
        skipped
    }

    fn eof(&self) -> bool {
        self.current_char + 1 >= self.string.len()
    }
}
pub struct Lexer<T: LexerString> {
    string: T,
}

impl<T: LexerString> Lexer<T> {
    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.string.skip_whitespace();
        let mut content: Vec<u8> = vec![];
        let mut token: Option<Token> = None;
        if self.string.get_current_char().is_ascii_digit() {
            content.extend(self.string.consume_char_type(u8::is_ascii_digit));
            token = Some(Token::Number(content));
        } else if self.string.get_current_char().is_ascii_alphabetic() {
            content.extend(self.string.consume_char_type(u8::is_ascii_alphabetic));
        } else if self.string.get_current_char().is_ascii_operator() {
            content.push(self.string.get_current_char());
            token = Some(Token::Operator(content));
            self.string.shift_chars();
        } else {
            return Err(LexerError::new());
        }
        Ok(token)
    }

    pub fn eof(&self) -> bool {
        self.string.eof()
    }
}

impl Lexer<VecLexerString> {
    pub fn new(str: &str) -> Lexer<VecLexerString> {
        Lexer {
            string: VecLexerString {
                string: Vec::<u8>::from(str),
                current_char: 0,
                next_char: 1,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_operators_and_numbers() {
        const TEST_STRING: &str = "124 + 238 +/34 -18";
        let mut lexer: Lexer<VecLexerString> = Lexer::new(TEST_STRING);
        let mut tokens: Vec<Token> = vec![];
        while !lexer.eof() {
            match lexer.next_token() {
                Err(_) => panic!("Lexer error"),
                Ok(None) => {}
                Ok(Some(token)) => tokens.push(token),
            }
        }
        println!("{:#?}", tokens);
        assert_eq!(
            tokens,
            vec![
                Token::Number(Vec::<u8>::from(b"124")),
                Token::Operator(Vec::<u8>::from(b"+")),
                Token::Number(Vec::<u8>::from(b"238")),
                Token::Operator(Vec::<u8>::from(b"+")),
                Token::Operator(Vec::<u8>::from(b"/")),
                Token::Number(Vec::<u8>::from(b"34")),
                Token::Operator(Vec::<u8>::from(b"-")),
                Token::Number(Vec::<u8>::from(b"18"))
            ]
        )
    }
}
