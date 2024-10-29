use crate::parser::{Program as ParserProgram, FunctionDeclaration, Statement, Exp, UnaryOp, Factor, BinaryOp};

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Negate,
    Complement,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Ampersand,
    Pipe,
    Caret,
    ShiftLeft,
    ShiftRight,
}

impl From<&UnaryOp> for UnaryOperator {
    fn from(op: &UnaryOp) -> Self {
        match op {
            UnaryOp::Negation => UnaryOperator::Negate,
            UnaryOp::Complement => UnaryOperator::Complement,
        }
    }
}

impl From<&BinaryOp> for BinaryOperator {
    fn from(op: &BinaryOp) -> Self {
        match op {
            BinaryOp::Add => BinaryOperator::Add,
            BinaryOp::Subtract => BinaryOperator::Subtract,
            BinaryOp::Multiply => BinaryOperator::Multiply,
            BinaryOp::Divide => BinaryOperator::Divide,
            BinaryOp::Modulo => BinaryOperator::Modulo,
            BinaryOp::BitwiseAnd => BinaryOperator::Ampersand,
            BinaryOp::BitwiseOr => BinaryOperator::Pipe,
            BinaryOp::BitwiseXor => BinaryOperator::Caret,
            BinaryOp::LeftShift => BinaryOperator::ShiftLeft,
            BinaryOp::RightShift => BinaryOperator::ShiftRight,
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
    Binary { operator: BinaryOperator, src1: Val, src2: Val, dst: Val },
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

impl Factor {
    fn generate_tac(&self, body: &mut Vec<Instruction>) -> Val {
        match self {
            Factor::Int(value) => Val::Constant(*value),
            Factor::Unary(op, exp) => {
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
            Factor::Exp(exp) => exp.generate_tac(body),
        }
    }
}

impl Exp {
    fn generate_tac(&self, body: &mut Vec<Instruction>) -> Val {
        match self {
            Exp::Factor(factor) => factor.generate_tac(body),
            Exp::Binary(left, op, right) => {
                let left_val = left.generate_tac(body);
                let right_val = right.generate_tac(body);
                let dst = Val::Identifier(format!("tmp.{}", body.len()));
                let instruction = Instruction::Binary {
                    operator: BinaryOperator::from(op),
                    src1: left_val,
                    src2: right_val,
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

impl FunctionDeclaration {
    pub fn generate_tac(&self) -> Function {
        let mut body = Vec::new();
        match self {
            FunctionDeclaration::Function(identifier, statement) => {
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