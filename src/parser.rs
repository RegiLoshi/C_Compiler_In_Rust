use crate::lex;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negation, // -
    Complement, // ~
    LogicalNot, // !
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    //Tag,
    Assignment,
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
pub enum FunctionDeclaration {
    Function(String, Statement),
}

#[derive(Debug)]
pub enum Program {
    Program(FunctionDeclaration),
}

pub enum Associativity{
    Left,
    Right,
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

impl PrettyPrint for FunctionDeclaration {
    fn pretty_print(&self, indent: usize) {
        match self {
            FunctionDeclaration::Function(name, statement) => {
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
        Some(n) if token.token_type != lex::TokenType::IDENTIFIER || token.value != n => {
            Err(format!("Expected identifier '{}', got '{}'", n, token.value))
        }
        None if token.token_type != lex::TokenType::IDENTIFIER => {
            Err(format!("Expected identifier, got '{}'", token.value))
        }
        _ => Ok(()),
    }
}

fn expect_token_type(token: &lex::Token, token_type: lex::TokenType) -> Result<(), String> {
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
        lex::TokenType::CONSTANT => Ok(Factor::Int(token.value.parse().unwrap())),
        lex::TokenType::NegationOp => {
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::Negation, Box::new(factor)))
        },
        lex::TokenType::TildeOp => {
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::Complement, Box::new(factor)))
        },
        lex::TokenType::LogicalNot => {
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::LogicalNot, Box::new(factor)))
        },
        lex::TokenType::OpenParen => {
            let exp = parse_expression(tokens, 0)?;
            if tokens.is_empty() {
                return Err("Unexpected end of file; expected closing parenthesis".to_string());
            }
            expect_token_type(&tokens.remove(0), lex::TokenType::CloseParen)?;
            Ok(Factor::Exp(Box::new(exp)))
        },
        _ => Err("Unexpected token while parsing factor".to_string()),
    }
}


fn get_operator_precedence(op: &BinaryOp) -> u8 {
    match op {
        BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => 50,
        BinaryOp::Add | BinaryOp::Subtract => 45,
        BinaryOp::LeftShift | BinaryOp::RightShift => 45,
        BinaryOp::BitwiseAnd => 44,
        BinaryOp::BitwiseXor => 43,
        BinaryOp::BitwiseOr => 42,
        BinaryOp::GreaterThan | BinaryOp::LessThan | BinaryOp::GreaterThanOrEqual | BinaryOp::LessThanOrEqual => 35,
        BinaryOp::Equal | BinaryOp::NotEqual => 30,
        BinaryOp::LogicalAnd => 10,
        BinaryOp::LogicalOr => 5,
        BinaryOp::Assignment => 1,
    }
}

// fn get_associativity(op: &BinaryOp) -> Associativity {
//     match op {
//         BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo |
//         BinaryOp::LeftShift | BinaryOp::RightShift | BinaryOp::BitwiseAnd | BinaryOp::BitwiseXor | BinaryOp::BitwiseOr => Associativity::Left,
//     }
// }

fn parse_op(token: &lex::Token) -> Result<BinaryOp, String> {
    match token.token_type {
        lex::TokenType::PLUS => Ok(BinaryOp::Add),
        lex::TokenType::NegationOp => Ok(BinaryOp::Subtract),
        lex::TokenType::STAR => Ok(BinaryOp::Multiply),
        lex::TokenType::SLASH => Ok(BinaryOp::Divide),
        lex::TokenType::MODULUS => Ok(BinaryOp::Modulo),
        lex::TokenType::AMPERSAND => Ok(BinaryOp::BitwiseAnd),
        lex::TokenType::PIPE => Ok(BinaryOp::BitwiseOr),
        lex::TokenType::CARET => Ok(BinaryOp::BitwiseXor),
        lex::TokenType::LeftShift => Ok(BinaryOp::LeftShift),
        lex::TokenType::RightShift => Ok(BinaryOp::RightShift),
        lex::TokenType::Equal => Ok(BinaryOp::Equal),
        lex::TokenType::NotEqual => Ok(BinaryOp::NotEqual),
        lex::TokenType::GreaterThan => Ok(BinaryOp::GreaterThan),
        lex::TokenType::LessThan => Ok(BinaryOp::LessThan),
        lex::TokenType::GreaterThanOrEqual => Ok(BinaryOp::GreaterThanOrEqual),
        lex::TokenType::LessThanOrEqual => Ok(BinaryOp::LessThanOrEqual),
        lex::TokenType::LogicalAnd => Ok(BinaryOp::LogicalAnd),
        lex::TokenType::LogicalOr => Ok(BinaryOp::LogicalOr),
        lex::TokenType::Assignment => Ok(BinaryOp::Assignment),
        _ => Err(format!("Unexpected token: {:?}", token)),
    }
}

fn parse_expression(tokens: &mut Vec<lex::Token>, min_precedence: u8) -> Result<Exp, String> {
    let mut left = Exp::Factor(parse_factor(tokens)?);

    while !tokens.is_empty() {
        let op = match parse_op(&tokens[0]) {
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
    expect_token_type(&tokens.remove(0), lex::TokenType::SEMICOLON)?;
    Ok(Statement::Return(exp))
}

fn parse_function_declaration(tokens: &mut Vec<lex::Token>) -> Result<FunctionDeclaration, String> {
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
    expect_token_type(&tokens.remove(0), lex::TokenType::OpenParen)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected 'void' or closing parenthesis".to_string());
    }
    expect_identifier(&tokens.remove(0), Some("void"))?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected closing parenthesis".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::CloseParen)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected opening brace".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::OpenBrace)?;
    let statement = parse_statement(tokens)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected closing brace".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::CloseBrace)?;
    if !tokens.is_empty() {
        return Err(format!("Unexpected token: {:?}", tokens[0]));
    }
    Ok(FunctionDeclaration::Function(name_token.value, statement))
}

pub fn parse_program(tokens: &mut Vec<lex::Token>) -> Result<Program, String> {
    if tokens.is_empty() {
        return Err("Empty program".to_string());
    }
    let func_decl = parse_function_declaration(tokens)?;
    Ok(Program::Program(func_decl))
}

