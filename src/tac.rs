use crate::parser::{Program as ParserProgram, Function_declaration, Statement, Exp, UnaryOp};

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Negate,
    Complement,
}

impl From<&UnaryOp> for UnaryOperator {
    fn from(op: &UnaryOp) -> Self {
        match op {
            UnaryOp::Negation => UnaryOperator::Negate,
            UnaryOp::Complement => UnaryOperator::Complement,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Val {
    Identifier(String),
    Constant(i32),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Return(Val),
    Unary { operator: UnaryOperator, src: Val, dst: Val },
}

#[derive(Clone, Debug)]
pub struct Function {
    pub identifier: String,
    pub body: Vec<Instruction>,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub function: Function,
}

impl Exp {
    fn generate_tac(&self, body: &mut Vec<Instruction>) -> Val {
        match self {
            Exp::Constant(value) => Val::Constant(*value),
            Exp::Unary(op, exp) => {
                let val = exp.generate_tac(body);
                let dst = Val::Identifier(format!("tmp.{}", body.len()));
                let instruction = Instruction::Unary {
                    operator: UnaryOperator::from(op),
                    src: val,
                    dst: dst.clone(),
                };
                body.push(instruction);
                dst
            }
        }
    }
}

impl Statement {
    fn generate_tac(&self, body: &mut Vec<Instruction>) {
        match self {
            Statement::Return(exp) => {
                let val = exp.generate_tac(body);
                body.push(Instruction::Return(val));
            }
        }
    }
}

impl Function_declaration {
    pub fn generate_tac(&self) -> Function {
        let mut body = Vec::new();
        match self {
            Function_declaration::Function(identifier, statement) => {
                statement.generate_tac(&mut body);
                Function {
                    identifier: identifier.clone(),
                    body,
                }
            }
        }
    }
}

impl ParserProgram {
    pub fn generate_tac(&self) -> Program {
        match self {
            ParserProgram::Program(func_decl) => {
                let function = func_decl.generate_tac();
                Program { function }
            }
        }
    }
    }

pub fn generate_tac(program: ParserProgram) -> Program {
    program.generate_tac()
}