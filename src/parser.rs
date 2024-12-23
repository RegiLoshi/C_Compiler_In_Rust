use crate::lex::{self};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Factor {
    Int(i32),
    Unary(UnaryOp, Box<Factor>),
    Exp(Box<Exp>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Var(String), // Variable name (identifier
    Factor(Factor), // Constant or parenthesized expression
    Binary(Box<Exp>, BinaryOp, Box<Exp>), // Binary operation
    Assignment(Box<Exp>, Box<Exp>) // Assignment
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Exp),
    Expression(Exp),
    Null,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Declaration(String, Option<Exp>),
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    D(Declaration),
    S(Statement),
}

#[derive(Debug, Clone)]
pub enum FunctionDeclaration {
    Function(String, Vec<Box<BlockItem>>),
}

#[derive(Debug, Clone)]
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
            },
            Exp::Var(name) => {
                println!("{}Variable: {}", " ".repeat(indent), name);
            },
            Exp::Assignment(left, right) => {
                println!("{}Assignment:", " ".repeat(indent));
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
            },
            Statement::Expression(exp) => {
                println!("{}Expression:", " ".repeat(indent));
                exp.pretty_print(indent + 2);
            },
            Statement::Null => {
                println!("{}Null", " ".repeat(indent));
            }
        }
    }
}

impl PrettyPrint for Declaration {
    fn pretty_print(&self, indent: usize) {
        match self {
            Declaration::Declaration(name, exp) => {
                println!("{}Declaration: {}", " ".repeat(indent), name);
                if let Some(exp) = exp {
                    exp.pretty_print(indent + 2);
                }
            }
        }
    }
}

impl PrettyPrint for BlockItem {
    fn pretty_print(&self, indent: usize) {
        match self {
            BlockItem::D(declaration) => {
                declaration.pretty_print(indent);
            }
            BlockItem::S(statement) => {
                statement.pretty_print(indent);
            }
        }
    }
}

impl PrettyPrint for FunctionDeclaration {
    fn pretty_print(&self, indent: usize) {
        match self {
            FunctionDeclaration::Function(name, block_items) => {
                println!("{}Function: {}", " ".repeat(indent), name);
                for item in block_items {
                    item.pretty_print(indent + 2);
                }
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

fn expect_main_keyword(token: &lex::Token) -> Result<(), String> {
    if token.value != "main" {
        return Err(format!("Expected main keyword, got '{}'", token.value));
    }
    Ok(())
}

fn expect_void_keyword(token: &lex::Token) -> Result<(), String> {
    if token.value != "void" {
        return Err(format!("Expected void keyword, got '{}'", token.value));
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

    // Clone the token value we need, so we don't keep a reference to tokens
    let token = tokens[0].clone();
    
    match token.token_type {
        // Case 1: Integer constant
        lex::TokenType::CONSTANT => {
            tokens.remove(0);
            Ok(Factor::Int(token.value.parse().unwrap()))
        },
        // Case 2: Identifier
        lex::TokenType::IDENTIFIER => {
            tokens.remove(0);
            Ok(Factor::Exp(Box::new(Exp::Var(token.value))))
        },
        // Case 3: Unary operators
        lex::TokenType::NegationOp => {
            tokens.remove(0);
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::Negation, Box::new(factor)))
        },
        lex::TokenType::TildeOp => {
            tokens.remove(0);
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::Complement, Box::new(factor)))
        },
        lex::TokenType::LogicalNot => {
            tokens.remove(0);
            let factor = parse_factor(tokens)?;
            Ok(Factor::Unary(UnaryOp::LogicalNot, Box::new(factor)))
        },
        // Case 4: Parenthesized expression
        lex::TokenType::OpenParen => {
            tokens.remove(0);
            let exp = parse_expression(tokens, 0)?;
            if tokens.is_empty() {
                return Err("Unexpected end of file; expected closing parenthesis".to_string());
            }
            expect_token_type(&tokens.remove(0), lex::TokenType::CloseParen)?;
            Ok(Factor::Exp(Box::new(exp)))
        },
        _ => Err(format!("Unexpected token while parsing factor: {:?}", token)),
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

        if tokens[0].token_type == lex::TokenType::Assignment {
            tokens.remove(0);
            let right = parse_expression(tokens, precedence)?;
            left = Exp::Assignment(Box::new(left), Box::new(right));
            continue;
        } else{
        tokens.remove(0);
        let right = parse_expression(tokens, precedence + 1)?;
        left = Exp::Binary(Box::new(left), op, Box::new(right));
        }
    }
    Ok(left)
}

fn parse_declaration(tokens: &mut Vec<lex::Token>) -> Result<Declaration, String> {
    // Check if we have any tokens left
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing declaration".to_string());
    }

    // Parse "int"
    let int_token = tokens.remove(0);
    expect_int_keyword(&int_token)?;

    // Parse identifier
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected identifier".to_string());
    }
    let name_token = tokens.remove(0);
    expect_identifier(&name_token, None)?;

    // Check for optional assignment
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected ';' or '='".to_string());
    }

    let next_token = &tokens[0];
    let exp = if next_token.token_type == lex::TokenType::Assignment {
        // Remove the '=' token
        tokens.remove(0);
        
        // Parse the expression
        if tokens.is_empty() {
            return Err("Unexpected end of file; expected expression after '='".to_string());
        }
        Some(parse_expression(tokens, 0)?)
    } else {
        None
    };

    // Parse semicolon
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected ';'".to_string());
    }
    let semicolon_token = tokens.remove(0);
    expect_token_type(&semicolon_token, lex::TokenType::SEMICOLON)?;

    Ok(Declaration::Declaration(name_token.value, exp))
}

fn parse_statement(tokens: &mut Vec<lex::Token>) -> Result<Statement, String> {
    // Check if we have any tokens
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing statement".to_string());
    }

    // Get first token without removing it
    let token = &tokens[0];

    match token.token_type {
        // Case 3: Just a semicolon
        lex::TokenType::SEMICOLON => {
            tokens.remove(0); // Remove semicolon
            Ok(Statement::Null)
        },
        // Case 1: Return statement
        lex::TokenType::KEYWORD if token.value == "return" => {
            tokens.remove(0); // Remove 'return'
            if tokens.is_empty() {
                return Err("Unexpected end of file after 'return'".to_string());
            }
            let exp = parse_expression(tokens, 0)?;
            if tokens.is_empty() {
                return Err("Unexpected end of file; expected semicolon".to_string());
            }
            expect_token_type(&tokens.remove(0), lex::TokenType::SEMICOLON)?;
            Ok(Statement::Return(exp))
        },
        // Case 2: Expression statement
        _ => {
            let exp = parse_expression(tokens, 0)?;
            if tokens.is_empty() {
                return Err("Unexpected end of file; expected semicolon".to_string());
            }
            expect_token_type(&tokens.remove(0), lex::TokenType::SEMICOLON)?;
            Ok(Statement::Expression(exp))
        }
    }
}

fn parse_block_items(tokens: &mut Vec<lex::Token>) -> Result<Box<BlockItem>, String> {
    if expect_int_keyword(&tokens[0]).is_ok(){
        let declaration = parse_declaration(tokens)?;
        Ok(Box::new(BlockItem::D(declaration)))
    } else {
        let statement = parse_statement(tokens)?;
        Ok(Box::new(BlockItem::S(statement)))
    }
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
    //expect_identifier(&name_token, Some("main"))?;
    expect_main_keyword(&name_token)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected opening parenthesis".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::OpenParen)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected 'void' or closing parenthesis".to_string());
    }
    // expect_identifier(&tokens.remove(0), Some("void"))?;
    expect_void_keyword(&tokens.remove(0))?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected closing parenthesis".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::CloseParen)?;
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected opening brace".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::OpenBrace)?;
    let mut block_items = Vec::new();
    while tokens[0].token_type != lex::TokenType::CloseBrace {
        block_items.push(parse_block_items(tokens)?);
    }
    if tokens.is_empty() {
        return Err("Unexpected end of file; expected closing brace".to_string());
    }
    expect_token_type(&tokens.remove(0), lex::TokenType::CloseBrace)?;
    if !tokens.is_empty() {
        return Err(format!("Unexpected token: {:?}", tokens[0]));
    }
    Ok(FunctionDeclaration::Function(name_token.value, block_items))
}

fn parse_program(tokens: &mut Vec<lex::Token>) -> Result<Program, String> {
    if tokens.is_empty() {
        return Err("Empty program".to_string());
    }
    let func_decl = parse_function_declaration(tokens)?;
    Ok(Program::Program(func_decl))
}
// Helper function to generate unique variable names
fn make_temporary(name: String, symbol_table: &HashMap<String, String>) -> String {
    let mut counter = 0;
    let mut temp_name = name.clone();
    while symbol_table.contains_key(&temp_name) {
        counter += 1;
        temp_name = format!("{}.{}", name, counter);
    }
    temp_name
}

// Expression resolution with improved error handling
fn resolve_expression(exp: Exp, symbol_table: &HashMap<String, String>) -> Result<Exp, String> {
    match exp {
        Exp::Assignment(left, right) => {
            let resolved_left = resolve_expression(*left, symbol_table)?;
            let resolved_right = resolve_expression(*right, symbol_table)?;
            
            // Extract the variable name from the resolved left expression
            let var_name = match &resolved_left {
                Exp::Var(_) => Ok(resolved_left),
                Exp::Factor(Factor::Exp(box_exp)) => {
                    match **box_exp {
                        Exp::Var(_) => Ok(resolved_left),
                        _ => Err("Left side of assignment must resolve to a variable".to_string())
                    }
                }
                _ => Err("Left side of assignment must resolve to a variable".to_string())
            }?;

            Ok(Exp::Assignment(Box::new(var_name), Box::new(resolved_right)))
        },
        Exp::Var(name) => {
            if !symbol_table.contains_key(&name) {
                return Err(format!("Variable '{}' not declared", name));
            }
            Ok(Exp::Var(symbol_table[&name].clone()))
        },
        Exp::Binary(left, op, right) => {
            let resolved_left = resolve_expression(*left, symbol_table)?;
            let resolved_right = resolve_expression(*right, symbol_table)?;
            Ok(Exp::Binary(Box::new(resolved_left), op, Box::new(resolved_right)))
        },
        Exp::Factor(factor) => {
            match factor {
                Factor::Int(value) => Ok(Exp::Factor(Factor::Int(value))),
                Factor::Unary(op, factor) => {
                    let resolved = resolve_expression(Exp::Factor(*factor), symbol_table)?;
                    match resolved {
                        Exp::Factor(f) => Ok(Exp::Factor(Factor::Unary(op, Box::new(f)))),
                        _ => Err("Expected a Factor after resolving unary expression".to_string())
                    }
                },
                Factor::Exp(exp) => {
                    let resolved = resolve_expression(*exp, symbol_table)?;
                    Ok(Exp::Factor(Factor::Exp(Box::new(resolved))))
                }
            }
        }
    }
}


// Declaration resolution with improved error handling
fn resolve_declaration(
    name: String, 
    init: Option<Exp>, 
    symbol_table: &mut HashMap<String, String>
) -> Result<Declaration, String> {
    // Check for redeclaration
    if symbol_table.contains_key(&name) {
        return Err(format!("Variable '{}' already declared", name));
    }

    // Generate unique identifier
    let unique_id = make_temporary(name.clone(), symbol_table);
    
    // Add to symbol table before resolving initialization
    symbol_table.insert(name, unique_id.clone());
    
    // Resolve initialization if present
    let resolved_init = match init {
        Some(init_exp) => Some(resolve_expression(init_exp, symbol_table)?),
        None => None
    };

    Ok(Declaration::Declaration(unique_id, resolved_init))
}

// Statement resolution with improved error handling
fn resolve_statement(statement: Statement, symbol_table: &HashMap<String, String>) -> Result<Statement, String> {
    match statement {
        Statement::Return(exp) => {
            let resolved_exp = resolve_expression(exp, symbol_table)?;
            Ok(Statement::Return(resolved_exp))
        },
        Statement::Expression(exp) => {
            let resolved_exp = resolve_expression(exp, symbol_table)?;
            Ok(Statement::Expression(resolved_exp))
        },
        Statement::Null => Ok(Statement::Null)
    }
}

// Block item resolution with proper error propagation
fn resolve_block_item(item: BlockItem, symbol_table: &mut HashMap<String, String>) -> Result<BlockItem, String> {
    match item {
        BlockItem::D(Declaration::Declaration(name, init)) => {
            let resolved = resolve_declaration(name, init, symbol_table)?;
            Ok(BlockItem::D(resolved))
        },
        BlockItem::S(statement) => {
            let resolved = resolve_statement(statement, symbol_table)?;
            Ok(BlockItem::S(resolved))
        }
    }
}

// Function declaration resolution with proper scope handling
fn resolve_function_declaration(func_decl: FunctionDeclaration) -> Result<FunctionDeclaration, String> {
    match func_decl {
        FunctionDeclaration::Function(name, block_items) => {
            let mut symbol_table = HashMap::new();
            let mut resolved_items = Vec::new();

            for item in block_items.into_iter() {
                let resolved_item = resolve_block_item(*item, &mut symbol_table)?;
                resolved_items.push(Box::new(resolved_item));
            }

            Ok(FunctionDeclaration::Function(name, resolved_items))
        }
    }
}

// Program resolution with proper error propagation
pub fn resolve_program(program: Program) -> Result<Program, String> {
    match program {
        Program::Program(func_decl) => {
            let resolved_func = resolve_function_declaration(func_decl)?;
            Ok(Program::Program(resolved_func))
        }
    }
}

// Main entry point for parsing and resolving
pub fn parse_and_resolve_program(tokens: &mut Vec<lex::Token>) -> Result<Program, String> {
    let parsed_program = parse_program(tokens)?;
    resolve_program(parsed_program)
}