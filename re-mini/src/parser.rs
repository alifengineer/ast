use crate::{ast::{Action, CmpOp, Condition, Expr, Op, Rule}, value::Value};

#[derive(Debug)]
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

impl Parser {
    fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        token
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos+=1;
        token
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        if !matches!(self.peek(), Some(Token::Rule)) {
            return Err("expected 'rule'".into());
        }
        self.advance();

        let name = if let Some(Token::Ident(s)) = self.advance() {
            s.clone()
        } else {return Err("expected rule name".into());};

        if matches!(self.peek(), Some(Token::StringLit(_))) {
            self.advance();
        }

        if !matches!(self.peek(), Some(Token::LBrace)) {
            return Err("expected {".into());
        }
        self.advance();

        if !matches!(self.peek(), Some(Token::When)) {
            return Err("expected 'when'".into());
        }
        self.advance();
        
        let condition = self.parse_condition()?;

        if !matches!(self.peek(), Some(Token::Then)) {
            return Err("expected 'then'".into());
        }
        self.advance();

        let actions = self.parse_actions()?;

        if !matches!(self.peek(), Some(Token::RBrace)) {
            return Err("expected '}'".into());
        }
        self.advance();

        Ok(Rule {name, condition, actions})
    }

    fn parse_condition(&mut self) -> Result<Condition, String> {
        let mut left = self.parse_comparison()?;
        loop {
            match self.peek() {
                Some(Token::And) => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    left = Condition::And(Box::new(left), Box::new(right));
                }
                Some(Token::Or) => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    left = Condition::Or(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_atom()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    let right = self.parse_atom()?;
                    left = Expr::BinOp {
                        left: Box::new(left),
                        op: Op::Add,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_actions(&mut self) -> Result<Vec<Action>, String> {
        let mut actions = Vec::new();
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            let name = if let Some(Token::Ident(s)) = self.advance() {
                s.clone()
            } else {
                return Err("expected identifier".into());
            };

            if !matches!(self.peek(), Some(Token::Dot)) {
                return  Err("expected '.'".into());
            }
            self.advance();

            let field_name = if let Some(Token::Ident(s)) = self.advance() {
                format!("{}.{}", name, s)
            } else {
                return Err("expected field name".into());
            };

            if !matches!(self.peek(), Some(Token::Assign)) {
                return Err("expected '='".into());
            }
            self.advance();

            let expr = self.parse_expr()?;
            

            if !matches!(self.peek(), Some(Token::Semicolon)) {
                return Err("expected ';'".into());
            }
            self.advance();

            actions.push(Action::Assign { field: field_name, expr: expr });
        }
        Ok(actions)
    }

    fn parse_comparison(&mut self) -> Result<Condition, String> {
        let left = self.parse_expr()?;
        let cmp_op = match self.peek() {
            Some(Token::Eq) => CmpOp::Eq,
            Some(Token::Lt) => CmpOp::Lt,
            Some(Token::Gt) => CmpOp::Gt,
            other => return Err(format!("unexpected operator {:?}", other)),
        };
        self.advance();
        let right = self.parse_expr()?;
        
        Ok(Condition::Compare {left, op:cmp_op, right})
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::Int(_)) => {
                if let Some(Token::Int(n)) = self.advance() {
                    let n = *n;
                    Ok(Expr::Literal(Value::Int(n)))
                } else {
                    unreachable!()
                }
            },
            Some(Token::Ident(_)) => {
                let name = if let Some(Token::Ident(s)) = self.advance() {
                    s.clone()
                } else {
                    unreachable!()
                };

                if matches!(self.peek(), Some(Token::Dot)) {
                    self.advance();
                    if let Some(Token::Ident(field)) = self.advance() {
                        Ok(Expr::FieldRef(format!("{}.{}", name, field)))
                    } else {
                        Err("expected field name after '.'".into())
                    }
                } else {
                    Ok(Expr::FieldRef(name))
                }
            },
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                if !matches!(self.peek(), Some(Token::RParen)) {
                    return Err("expected ')'".to_string())
                }
                self.advance();
                Ok(expr)
            }
            other => Err(format!("unexpected token in atom {:?}", other)),
        }
    } 
}

fn parse(input: String) -> Result<Vec<Rule>, String> {
    let tokens = tokenize(input.to_string())?;
    let mut parser = Parser{tokens, pos: 0};
    let mut rules = vec![];
    while parser.pos < parser.tokens.len() {
        rules.push(parser.parse_rule()?);
    }

    Ok(rules)
}


#[cfg(test)]
mod tests {
    use crate::context::DataContext;

    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("Vibo.A == 0".to_string()).unwrap();
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn test_parse_fib_rule() {
        let input = r#"
    rule CalcFib "Calculate Fibonacci" {
        when
            Vibo.A == 0 || Vibo.B == 0
        then
            Vibo.A = 1;
            Vibo.B = 1;
    }
    "#;

        let rules = parse(input.to_string()).unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "CalcFib");

        let mut ctx = DataContext::new();
        ctx.set("Vibo.A".into(), Value::Int(0));
        ctx.set("Vibo.B".into(), Value::Int(1));

        for rule in rules {
            let run = rule.evaluate(&ctx).unwrap();
            assert_eq!(run, true);

            assert_eq!(rule.execute(&mut ctx).unwrap(), ());
           
            assert_eq!(ctx.get("Vibo.A".into()).unwrap().clone(), Value::Int(1));
            assert_eq!(ctx.get("Vibo.B".into()).unwrap().clone(), Value::Int(1));
        }
    }
}
