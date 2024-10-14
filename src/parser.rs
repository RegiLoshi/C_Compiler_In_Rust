use crate::lex;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negation,
    Complement,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum Factor {
    Int(i32),
    Unary(UnaryOp, Box<Factor>),
    Exp(Box<Exp>),
}

#[derive(Debug)]
pub enum Exp {
    Factor(Factor),
    Binary(Box<Exp>, BinaryOp, Box<Exp>),
}

#[derive(Debug)]
pub enum Statement {
    Return(Exp),
}

#[derive(Debug)]
pub enum Function_declaration {
    Function(String, Statement),
}

#[derive(Debug)]
pub enum Program {
    Program(Function_declaration),
}

pub trait PrettyPrint {
    fn pretty_print(&self, indent: usize);
}

impl PrettyPrint for Factor {
    fn pretty_print(&self, indent: usize) {
        match self {
            Factor::Int(value) => {
                println!("{}Int: {}", " ".repeat(indent), value);
            }
            Factor::Unary(op, factor) => {
                println!("{}Unary Operation: {:?}", " ".repeat(indent), op);
                factor.pretty_print(indent + 2);
            }
            Factor::Exp(exp) => {
                println!("{}Parenthesized Expression:", " ".repeat(indent));
                exp.pretty_print(indent + 2);
            }
        }
    }
}

impl PrettyPrint for Exp {
    fn pretty_print(&self, indent: usize) {
        match self {
            Exp::Factor(factor) => factor.pretty_print(indent),
            Exp::Binary(left, op, right) => {
                println!("{}Binary Operation: {:?}", " ".repeat(indent), op);
                left.pretty_print(indent + 2);
                right.pretty_print(indent + 2);
            }
        }
    }
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, indent: usize) {
        match self {
            Statement::Return(exp) => {
                println!("{}Return:", " ".repeat(indent));
                exp.pretty_print(indent + 2);
            }
        }
    }
}

impl PrettyPrint for Function_declaration {
    fn pretty_print(&self, indent: usize) {
        match self {
            Function_declaration::Function(name, statement) => {
                println!("{}Function: {}", " ".repeat(indent), name);
                statement.pretty_print(indent + 2);
            }
        }
    }
}

impl PrettyPrint for Program {
    fn pretty_print(&self, indent: usize) {
        match self {
            Program::Program(func_decl) => {
                println!("{}Program:", " ".repeat(indent));
                func_decl.pretty_print(indent + 2);
            }
        }
    }
}

fn expect_int_keyword(token: &lex::Token) -> Result<(), String> {
    if token.value != "int" {
        return Err(format!("Expected int keyword, got '{}'", token.value));
    }
    Ok(())
}

fn expect_identifier(token: &lex::Token, expected: Option<&str>) -> Result<(), String> {
    match expected {
        Some(n) if token.token_type != lex::TOKEN_TYPE::IDENTIFIER || token.value != n => {
            Err(format!("Expected identifier '{}', got '{}'", n, token.value))
        }
        None if token.token_type != lex::TOKEN_TYPE::IDENTIFIER => {
            Err(format!("Expected identifier, got '{}'", token.value))
        }
        _ => Ok(()),
    }
}

fn expect_token_type(token: &lex::Token, token_type: lex::TOKEN_TYPE) -> Result<(), String> {
    if token.token_type != token_type {
        return Err(format!("Expected token type {:?}, got {:?}", token_type, token.token_type));
    }
    Ok(())
}

fn parse_factor(tokens: &mut Vec<lex::Token>) -> Result<Factor, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing factor".to_string());
    }
    let token = tokens.remove(0);
    match token.token_type {
        lex::TOKEN_TYPE::CONSTANT => Ok(Factor::Int(token.value.parse().unwrap())),
        lex::TOKEN_TYPE::NEGATION_OP => {
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::Negation, Box::new(factor)))
        }
        lex::TOKEN_TYPE::TILDE_OP => {
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::Complement, Box::new(factor)))
        }
        lex::TOKEN_TYPE::OPEN_PAREN => {
            let exp = parse_expression(tokens, 0)?;
            if tokens.is_empty() {
                return Err("Unexpected end of file; expected closing parenthesis".to_string());
            }
            expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::CLOSE_PAREN)?;
            Ok(Factor::Exp(Box::new(exp)))
        }
        _ => Err("Unexpected token while parsing factor".to_string()),
    }
}

fn get_operator_precedence(op: &BinaryOp) -> u8 {
    match op {
        BinaryOp::Add | BinaryOp::Subtract => 45,
        BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => 50,
    }
}

fn parse_binary_op(token: &lex::Token) -> Result<BinaryOp, String> {
    match token.token_type {
        lex::TOKEN_TYPE::PLUS => Ok(BinaryOp::Add),
        lex::TOKEN_TYPE::NEGATION_OP => Ok(BinaryOp::Subtract),
        lex::TOKEN_TYPE::STAR => Ok(BinaryOp::Multiply),
        lex::TOKEN_TYPE::SLASH => Ok(BinaryOp::Divide),
        lex::TOKEN_TYPE::MODULUS => Ok(BinaryOp::Modulo),
        _ => Err(format!("Unexpected token: {:?}", token)),
    }
}

fn parse_expression(tokens: &mut Vec<lex::Token>, min_precedence: u8) -> Result<Exp, String> {
    let mut left = Exp::Factor(parse_factor(tokens)?);

    while !tokens.is_empty() {
        let op = match parse_binary_op(&tokens[0]) {
            Ok(op) => op,
            Err(_) => break,
        };

        let precedence = get_operator_precedence(&op);
        if precedence < min_precedence {
            break;
        }

        tokens.remove(0);
        let right = parse_expression(tokens, precedence + 1)?;
        left = Exp::Binary(Box::new(left), op, Box::new(right));
    }

    Ok(left)
}

fn parse_statement(tokens: &mut Vec<lex::Token>) -> Result<Statement, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing statement".to_string());
    }
    expect_identifier(&tokens.remove(0), Some("return"))?;
    let exp = parse_expression(tokens, 0)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected semicolon".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::SEMICOLON)?;
    Ok(Statement::Return(exp))
}

fn parse_function_declaration(tokens: &mut Vec<lex::Token>) -> Result<Function_declaration, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing function declaration".to_string());
    }
    expect_int_keyword(&tokens.remove(0))?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected function name".to_string());
    }
    let name_token = tokens.remove(0);
    expect_identifier(&name_token, Some("main"))?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected opening parenthesis".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::OPEN_PAREN)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected 'void' or closing parenthesis".to_string());
    }
    expect_identifier(&tokens.remove(0), Some("void"))?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected closing parenthesis".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::CLOSE_PAREN)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected opening brace".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::OPEN_BRACE)?;
    let statement = parse_statement(tokens)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected closing brace".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::CLOSE_BRACE)?;
    if !tokens.is_empty() {
        return Err(format!("Unexpected token: {:?}", tokens[0]));
    }
    Ok(Function_declaration::Function(name_token.value, statement))
}

pub fn parse_program(tokens: &mut Vec<lex::Token>) -> Result<Program, String> {
    if tokens.is_empty() {
        return Err("Empty program".to_string());
    }
    let func_decl = parse_function_declaration(tokens)?;
    Ok(Program::Program(func_decl))
}

// fn main() {
//     let text = "int main( {
//         return 0;
//     }";
//     let mut lexer = lex::Lex::new(text);
//     let mut tokens = lexer.get_tokens();


//     //remove comments
//     let mut filtered_tokens: Vec<_> = tokens
//         .into_iter()
//         .filter(|token| token.token_type != lex::TOKEN_TYPE::COMMENT)
//         .collect();
    
//     // println!("Tokens: {:?}", filtered_tokens);

//     match parse_program(&mut filtered_tokens) {
//         Ok(program) => {
//             println!("Parsed successfully. Pretty print:");
//             program.pretty_print(0);
//         }
//         Err(e) => {
//             eprintln!("Parsing error: {}", e);
//             std::process::exit(1);
//         }
//     }
// }