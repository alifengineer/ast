use std::fmt::{format, Error};

use crate::value::Value;

enum Token {
    Int(i64),
    Bool(bool),

    Ident(String),

    Rule,
    When,
    Then,

    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Dot,

    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Assign,
    And,
    Or,
    Plus,
    Minus,
    Mul,
    Div,

    StringLit(String),
}

fn tokenize(input: String) -> Result<Vec<Token>, String> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    let mut tokens = Vec::new();

    while pos < bytes.len() {
        if bytes[pos].is_ascii_whitespace() {
            pos += 1;
            continue;
        }

        match bytes[pos] {
            b'{' => {
                tokens.push(Token::LBrace);
                pos += 1;
                continue;
            }
            b'}' => {
                tokens.push(Token::RBrace);
                pos += 1;
                continue;
            }
            b'(' => {
                tokens.push(Token::LParen);
                pos += 1;
                continue;
            }
            b')' => {
                tokens.push(Token::RParen);
                pos += 1;
                continue;
            }
            b';' => {
                tokens.push(Token::Semicolon);
                pos += 1;
                continue;
            }
            b'.' => {
                tokens.push(Token::Dot);
                pos += 1;
                continue;
            }
            b'+' => {
                tokens.push(Token::Plus);
                pos += 1;
                continue;
            }
            b'-' => {
                tokens.push(Token::Minus);
                pos += 1;
                continue;
            }
            b'*' => {
                tokens.push(Token::Mul);
                pos += 1;
                continue;
            }
            _ => {
                let next = peek(bytes, pos + 1);
                match bytes[pos] {
                    b'=' => {
                        if next == Some(b'=') {
                            tokens.push(Token::Eq);
                            pos += 2;
                        } else {
                            tokens.push(Token::Assign);
                            pos += 1;
                        }
                        continue;
                    }
                    b'!' => {
                        if next == Some(b'=') {
                            tokens.push(Token::NotEq);
                            pos += 2;
                        } else {
                            return Err(format!("unexpected char {} at pos {}", bytes[pos], pos));
                        }
                        continue;
                    }
                    b'<' => {
                        if next == Some(b'=') {
                            tokens.push(Token::LtEq);
                            pos += 2;
                        } else {
                            tokens.push(Token::Lt);
                            pos += 1;
                        }
                        continue;
                    }
                    b'>' => {
                        if next == Some(b'=') {
                            tokens.push(Token::GtEq);
                            pos += 2;
                        } else {
                            tokens.push(Token::Gt);
                            pos += 1;
                        }
                        continue;
                    }
                    b'&' => {
                        if next == Some(b'&') {
                            tokens.push(Token::And);
                            pos += 2;
                        } else {
                            return Err(format!("unexpected char {} at pos {}", bytes[pos], pos));
                        }
                        continue;
                    }
                    b'|' => {
                        if next == Some(b'|') {
                            tokens.push(Token::Or);
                            pos += 2;
                        } else {
                            return Err(format!("unexpected char {} at pos {}", bytes[pos], pos));
                        }
                        continue;
                    }
                    _ => {
                        if bytes[pos] == b'/' {
                            let next = peek(bytes, pos + 1);
                            if next == Some(b'/') {
                                pos += 2;
                                while pos < bytes.len() && bytes[pos] != b'\n' {
                                    pos += 1;
                                }
                                continue;
                            } else if next == Some(b'*') {
                                pos += 2;
                                while pos + 1 < bytes.len() {
                                    if bytes[pos] == b'*' && bytes[pos + 1] == b'/' {
                                        pos += 2;
                                        break;
                                    }
                                    pos += 1;
                                }
                                continue;
                            } else {
                                tokens.push(Token::Div);
                                pos += 1;
                                continue;
                            }
                        }

                        if bytes[pos] == b'"' {
                            pos += 1;
                            let start = pos;
                            while pos < bytes.len() && bytes[pos] != b'"' {
                                pos += 1;
                            }
                            let s = input[start..pos].to_string();
                            tokens.push(Token::StringLit(s));
                            pos += 1;
                            continue;
                        }

                        if bytes[pos].is_ascii_digit() {
                            let start = pos;
                            while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                                pos += 1;
                            }
                            let n: i64 = input[start..pos].parse().map_err(|e| format!("{}", e))?;
                            tokens.push(Token::Int(n));
                            continue;
                        }

                        if bytes[pos].is_ascii_alphabetic() || bytes[pos] == b'_' {
                            let start = pos;
                            while pos < bytes.len()
                                && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_')
                            {
                                pos += 1;
                            }

                            let word = &input[start..pos];
                            match word {
                                "rule" => tokens.push(Token::Rule),
                                "when" => tokens.push(Token::When),
                                "then" => tokens.push(Token::Then),
                                "true" => tokens.push(Token::Bool(true)),
                                "false" => tokens.push(Token::Bool(false)),
                                _ => tokens.push(Token::Ident(word.to_string())),
                            }
                            continue;
                        }

                        return Err(format!("unexpected char {} at pos {}", bytes[pos], pos));
                    }
                }
            }
        }
    }
    Ok(tokens)
}

fn peek(bytes: &[u8], pos: usize) -> Option<u8> {
    bytes.get(pos).copied()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("Vibo.A == 0".to_string()).unwrap();
        assert_eq!(tokens.len(), 5);
    }
}
