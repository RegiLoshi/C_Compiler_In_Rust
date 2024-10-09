use crate::lex;

#[derive(Debug)]
pub enum UnaryOp {
    Negation,
    Complement,
}


#[derive(Debug)]
pub enum Exp {
    Constant(i32),
    Unary(UnaryOp, Box<Exp>),
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

impl PrettyPrint for Exp {
    fn pretty_print(&self, indent: usize) {
        match self {
            Exp::Constant(value) => {
                println!("{}Constant: {}", " ".repeat(indent), value);
            }
            Exp::Unary(op, exp) => {
                println!("{}Unary Operation: {:?}", " ".repeat(indent), op);
                exp.pretty_print(indent + 2);
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
fn parse_expression(tokens: &mut Vec<lex::Token>) -> Result<Exp, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing expression".to_string());
    }
    let token = tokens.remove(0);
    if token.token_type == lex::TOKEN_TYPE::CONSTANT {
        return Ok(Exp::Constant(token.value.parse().unwrap()));
    }else if token.token_type == lex::TOKEN_TYPE::NEGATION_OP{
        let exp = parse_expression(tokens)?;
        return Ok(Exp::Unary(UnaryOp::Negation, Box::new(exp)));
    }else if token.token_type == lex::TOKEN_TYPE::TILDE_OP{
        let exp = parse_expression(tokens)?;
        return Ok(Exp::Unary(UnaryOp::Complement, Box::new(exp)));
    }else if token.token_type == lex::TOKEN_TYPE::OPEN_PAREN{
        let exp = parse_expression(tokens)?;
        if tokens.is_empty() {
            return Err("Unexpected end of file; expected closing parenthesis".to_string());
        }
        expect_token_type(&tokens.remove(0), lex::TOKEN_TYPE::CLOSE_PAREN)?;
        return Ok(exp);
    }else{
        return Err("Unexpected token while parsing expression".to_string());
    }
}

fn parse_statement(tokens: &mut Vec<lex::Token>) -> Result<Statement, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of file while parsing statement".to_string());
    }
    expect_identifier(&tokens.remove(0), Some("return"))?;
    let exp = parse_expression(tokens)?;
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