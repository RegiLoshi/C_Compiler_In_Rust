use std::process;
#[derive(Debug, PartialEq)]
pub enum TokenType {
    IDENTIFIER, 
    CONSTANT,
    KEYWORD,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SEMICOLON,
    SLASH,
    COMMENT,
    LongComment,
    STAR,
    PLUS,
    MODULUS,
    TildeOp,
    NegationOp,
    DecrementOp,
    AMPERSAND,
    PIPE,
    CARET,
    LessThan,
    GreaterThan,
    LeftShift,
    RightShift,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Lex<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Lex<'a> {
    pub fn new(text: &str) -> Lex {
        Lex { text, pos: 0 }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_whitespace() {
            self.advance();
        }
    
        if self.pos >= self.text.len() {
            return; 
        }
    }

    

    fn number(&mut self) -> Token {
        let mut result = String::new();
        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_digit(10) {
            result.push(self.text.chars().nth(self.pos).unwrap());
            self.advance();
        }
        Token { token_type: TokenType::CONSTANT, value: result }
    }

    fn identifier(&mut self) -> Token {
        let mut result = String::new();
        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_alphanumeric() {
            result.push(self.text.chars().nth(self.pos).unwrap());
            self.advance();
        }
        Token { token_type: TokenType::IDENTIFIER, value: result }
    }

    fn next(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();
    
        if self.pos >= self.text.len() {
            return Ok(None); 
        } 
        match self.text.chars().nth(self.pos).unwrap() {
            '0'..='9' => {
                let num_token = self.number();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_alphabetic() {
                    return Err(format!("Invalid constant followed by identifier at position {}: '{}'", self.pos, self.text));
                }
                Ok(Some(num_token))
            },
            'a'..='z' | 'A'..='Z' => Ok(Some(self.identifier())),
            '(' => { self.advance(); Ok(Some(Token { token_type: TokenType::OpenParen, value: "(".to_string() })) },
            ')' => { self.advance(); Ok(Some(Token { token_type: TokenType::CloseParen, value: ")".to_string() })) },
            '{' => { self.advance(); Ok(Some(Token { token_type: TokenType::OpenBrace, value: "{".to_string() })) },
            '}' => { self.advance(); Ok(Some(Token { token_type: TokenType::CloseBrace, value: "}".to_string() })) },
            ';' => { self.advance(); Ok(Some(Token { token_type: TokenType::SEMICOLON, value: ";".to_string() })) },
            '/' => { 
                self.advance();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '/' {
                    self.advance();
                    while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() != '\n' {
                        self.advance();
                    }
                    Ok(Some(Token { token_type: TokenType::COMMENT, value: "//".to_string() }))
                } else if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '*' {
                    self.advance();
                    let mut long_comment = "/*".to_string();
                    while self.pos < self.text.len() {
                        long_comment.push(self.text.chars().nth(self.pos).unwrap());
                        if self.text.chars().nth(self.pos).unwrap() == '*' {
                            self.advance();
                            if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '/' {
                                long_comment.push('/');
                                self.advance();
                                break;
                            }
                        } else {
                            self.advance();
                        }
                    }
                    Ok(Some(Token { token_type: TokenType::LongComment, value: long_comment }))
                } else {
                    Ok(Some(Token { token_type: TokenType::SLASH, value: "/".to_string() }))
                }
            },
            '*' => { self.advance(); Ok(Some(Token { token_type: TokenType::STAR, value: "*".to_string() })) },
            '~' => { self.advance(); Ok(Some(Token { token_type: TokenType::TildeOp, value: "~".to_string() })) },
            '-' => {
                self.advance();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '-' {
                    self.advance();
                    Ok(Some(Token { token_type: TokenType::DecrementOp, value: "--".to_string() }))
                } else {
                    Ok(Some(Token { token_type: TokenType::NegationOp, value: "-".to_string() }))
                }
            },
            '%' => { self.advance(); Ok(Some(Token { token_type: TokenType::MODULUS, value: "%".to_string() })) },
            '+' => { self.advance(); Ok(Some(Token { token_type: TokenType::PLUS, value: "+".to_string() })) },
            '&' => { self.advance(); Ok(Some(Token { token_type: TokenType::AMPERSAND, value: "&".to_string() })) },
            '|' => { self.advance(); Ok(Some(Token { token_type: TokenType::PIPE, value: "|".to_string() })) },
            '^' => { self.advance(); Ok(Some(Token { token_type: TokenType::CARET, value: "^".to_string() })) },
            '<' => {
                self.advance();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '<' {
                    self.advance();
                    Ok(Some(Token { token_type: TokenType::LeftShift, value: "<<".to_string() }))
                } else {
                    Ok(Some(Token { token_type: TokenType::LessThan, value: "<".to_string() }))
                }
            },
            '>' => {
                self.advance();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '>' {
                    self.advance();
                    Ok(Some(Token { token_type: TokenType::RightShift, value: ">>".to_string() }))
                } else {
                    Ok(Some(Token { token_type: TokenType::GreaterThan, value: ">".to_string() }))
                }
            },
            _ => Err(format!("Invalid character '{}' found at position {} in text '{}'", 
                            self.text.chars().nth(self.pos).unwrap(), self.pos, self.text)),
        }
    }    

    pub fn get_tokens(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    while self.pos < self.text.len() {
        match self.next() {
            Ok(Some(token)) => tokens.push(token),
            Ok(None) => break,
             Err(err) => {
                eprintln!("Lexing error: {}", err);
                process::exit(1); 
            }
        }
    }
    tokens
}

    
}

