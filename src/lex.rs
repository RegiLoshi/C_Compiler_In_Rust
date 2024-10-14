use std::process;
#[derive(Debug, PartialEq)]
pub enum TOKEN_TYPE {
    IDENTIFIER, 
    CONSTANT,
    KEYWORD,
    OPEN_PAREN,
    CLOSE_PAREN,
    OPEN_BRACE,
    CLOSE_BRACE,
    SEMICOLON,
    SLASH,
    COMMENT,
    LONG_COMMENT,
    STAR,
    PLUS,
    MODULUS,
    TILDE_OP,
    NEGATION_OP,
    DECREMENT_OP,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TOKEN_TYPE,
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
        Token { token_type: TOKEN_TYPE::CONSTANT, value: result }
    }

    fn identifier(&mut self) -> Token {
        let mut result = String::new();
        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_alphanumeric() {
            result.push(self.text.chars().nth(self.pos).unwrap());
            self.advance();
        }
        Token { token_type: TOKEN_TYPE::IDENTIFIER, value: result }
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
            '(' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::OPEN_PAREN, value: "(".to_string() })) },
            ')' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::CLOSE_PAREN, value: ")".to_string() })) },
            '{' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::OPEN_BRACE, value: "{".to_string() })) },
            '}' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::CLOSE_BRACE, value: "}".to_string() })) },
            ';' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::SEMICOLON, value: ";".to_string() })) },
            '/' => { 
                self.advance();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '/' {
                    self.advance();
                    while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() != '\n' {
                        self.advance();
                    }
                    Ok(Some(Token { token_type: TOKEN_TYPE::COMMENT, value: "//".to_string() }))
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
                    Ok(Some(Token { token_type: TOKEN_TYPE::LONG_COMMENT, value: long_comment }))
                } else {
                    Ok(Some(Token { token_type: TOKEN_TYPE::SLASH, value: "/".to_string() }))
                }
            },
            '*' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::STAR, value: "*".to_string() })) },
            '~' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::TILDE_OP, value: "~".to_string() })) },
            '-' => {
                self.advance();
                if self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap() == '-' {
                    self.advance();
                    Ok(Some(Token { token_type: TOKEN_TYPE::DECREMENT_OP, value: "--".to_string() }))
                } else {
                    Ok(Some(Token { token_type: TOKEN_TYPE::NEGATION_OP, value: "-".to_string() }))
                }
            },
            '%' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::MODULUS, value: "%".to_string() })) },
            '+' => { self.advance(); Ok(Some(Token { token_type: TOKEN_TYPE::PLUS, value: "+".to_string() })) },
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


// use std::fs;
// use std::env;
// use std::process;
// use std::path::Path;

// fn main() {
//     let text = "int main(void) {
//         return 1foo;
//     }";
//     let mut lexer = Lex::new(text);
//     let tokens = lexer.get_tokens();
//     for token in tokens {
//         println!("{:?}", token);
//     }
// }


//     let args: Vec<String> = env::args().collect();
    
//     // Print out all arguments for debugging
//     eprintln!("Received {} arguments:", args.len());
//     for (i, arg) in args.iter().enumerate() {
//         eprintln!("Argument {}: {}", i, arg);
//     }

//     let input_file = &args[2];
//     eprintln!("Attempting to read file: {}", input_file);

//     // Read the input file
//     let input = match fs::read_to_string(input_file) {
//         Ok(content) => content,
//         Err(err) => {
//             eprintln!("Failed to read input file: {}", err);
//             process::exit(1);
//         }
//     };

//     // Create a lexer instance and get tokens
//     let mut lexer = Lex::new(&input);
//     let tokens = lexer.get_tokens();
//     for token in tokens {
//         println!("{:?}", token);
//     }
// }