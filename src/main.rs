#![allow(unused_imports)]

use std::{fs, os, sync::Mutex};

#[derive(Debug, Clone)]
enum TokenType {
    Identifier(String),
    Boolean(bool),
    Number(i32),
    Operator(char),
    Delimiter(char),
    Keyword(String),
    Comment(String),
    EOF,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
    index: usize,
}

fn main() {
    // Apply Lexical Analysis
    let tokens = lex("example.protodecl").unwrap();

    println!("{:?}", tokens);
}

fn lex(_file: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut lex = Lexer::new(_file.to_string())?;

    loop {
        let token = lex.next_token();
        match token {
            Some(t) => {
                println!("{:?}", t);
                tokens.push(t);
            }
            None => {
                println!("EOF");
                break;
            }
        }
    }
    Ok(tokens)
}

struct Lexer {
    filename: String,
    data: Vec<char>,
    line: usize,
    column: usize,
    position: usize,
    cursor: usize,

    current_char: char,
    last_token: Option<Token>,
}

impl Lexer {
    fn new(file: String) -> Result<Lexer, String> {
        let data = match fs::read_to_string(&file) {
            Ok(data) => data,
            Err(e) => return Err(format!("{}", e)),
        };

        let mut lexer = Lexer {
            filename: file,
            data: data.chars().collect(),
            line: 1,
            column: 1,
            position: 0,
            cursor: 0,
            current_char: '\n',
            last_token: None,
        };

        lexer._readchar();

        Ok(lexer)
    }

    fn _new_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            line: self.line,
            column: self.column,
            index: self.cursor,
        }
    }

    fn _readchar(&mut self) -> bool {
        if self.cursor >= self.data.len() {
            self.last_token = Some(self._new_token(TokenType::EOF));
            return false;
        } else {
            self.current_char = self.data[self.cursor];
            if self.current_char == '\n' {
                self.line += 1;
                self.column = 0;
            }
        }
        self.position = self.cursor;
        self.cursor += 1;
        self.column += 1;

        true
    }

    fn _skip_whitespace(&mut self) -> bool {
        while self.current_char.is_whitespace() {
            if !self._readchar() {
                return false;
            }
        }
        true
    }

    fn _read_identifier(&mut self) -> String {
        let position = self.position;
        while self.current_char.is_alphanumeric() {
            if !self._readchar() {
                break;
            }
        }
        self.data[position..self.position].iter().collect()
    }

    fn _nextchar(&mut self) -> Option<char> {
        if self.cursor >= self.data.len() {
            return None;
        }
        Some(self.data[self.cursor])
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if !self._skip_whitespace() {
            return None;
        }

        let mut token = None;

        match self.current_char {
            '/' => match self._nextchar() {
                Some('/') => {
                    // Skip /
                    self._readchar();
                    self._readchar();

                    // Read until end of line
                    let position = self.position;
                    while self.current_char != '\n' {
                        if !self._readchar() {
                            break;
                        }
                    }
                    token = Some(self._new_token(TokenType::Comment(
                        self.data[position..self.position].iter().collect(),
                    )));
                    self._readchar();
                }
                Some('*') => {
                    // Skip *
                    self._readchar();
                    self._readchar();

                    // Read until */
                    let position = self.position;
                    while self.current_char != '*' || self._nextchar() != Some('/') {
                        if !self._readchar() {
                            break;
                        }
                    }
                    self._readchar();
                    self._readchar();

                    token = Some(self._new_token(TokenType::Comment(
                        self.data[position..self.position].iter().collect(),
                    )));
                    self._readchar();
                }
                _ => {
                    self.last_token = Some(self._new_token(TokenType::Operator('/')));
                }
            },

            '+' | '-' | '*' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' => {
                self.last_token = Some(self._new_token(TokenType::Operator(self.current_char)));
                self._readchar();
            }

            '{' | '}' | '(' | ')' | '[' | ']' | ';' => {
                token = Some(self._new_token(TokenType::Delimiter(self.current_char)));
                self._readchar();
            }

            _ => {
                let identifier = self._read_identifier();

                match identifier.as_str() {
                    "enum" | "packet" | "protocol" | "message" | "field" | "bool" | "u8"
                    | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128"
                    | "CString" | "String" | "Cbytes" | "Bytes" | "Bytes8le" | "Bytes16le"
                    | "Bytes32le" | "Bytes64le" | "Bytes8be" | "Bytes16be" | "Bytes32be"
                    | "Bytes64be" | "String8le" | "String16le" | "String32le" | "String64le"
                    | "String8be" | "String16be" | "String32be" | "String64be" | "Array"
                    | "Padding" | "Bits" | "f32" | "f64" => {
                        token = Some(self._new_token(TokenType::Keyword(identifier)));
                    }

                    "true" => {
                        token = Some(self._new_token(TokenType::Boolean(true)));
                    }
                    "false" => {
                        token = Some(self._new_token(TokenType::Boolean(false)));
                    }

                    _ => {
                        token = Some(self._new_token(TokenType::Identifier(identifier)));
                    }
                }

                self._readchar();
            }
        }

        match token {
            Some(x) => {
                self.last_token = Some(x.clone());
                return Some(x);
            }
            None => {
                self.last_token = None;
                return None;
            }
        }
    }
}
