// use crate::parser::{Program as ParserProgram, FunctionDeclaration, Statement, Exp, UnaryOp, Factor, BinaryOp};

// #[derive(Clone, Debug)]
// pub enum UnaryOperator {
//     Negate,
//     Complement,
//     LogicalNot
// }

// #[derive(Clone, Debug)]
// pub enum BinaryOperator {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     Modulo,
//     Ampersand,
//     Pipe,
//     Caret,
//     ShiftLeft,
//     ShiftRight,
//     LogicalAnd,
//     LogicalOr,
//     Equal,
//     NotEqual,
//     GreaterThan,
//     GreaterThanOrEqual,
//     LessThan,
//     LessThanOrEqual,
//     Assign,
// }

// impl From<&UnaryOp> for UnaryOperator {
//     fn from(op: &UnaryOp) -> Self {
//         match op {
//             UnaryOp::Negation => UnaryOperator::Negate,
//             UnaryOp::Complement => UnaryOperator::Complement,
//             UnaryOp::LogicalNot => UnaryOperator::LogicalNot,
//         }
//     }
// }

// impl From<&BinaryOp> for BinaryOperator {
//     fn from(op: &BinaryOp) -> Self {
//         match op {
//             BinaryOp::Add => BinaryOperator::Add,
//             BinaryOp::Subtract => BinaryOperator::Subtract,
//             BinaryOp::Multiply => BinaryOperator::Multiply,
//             BinaryOp::Divide => BinaryOperator::Divide,
//             BinaryOp::Modulo => BinaryOperator::Modulo,
//             BinaryOp::BitwiseAnd => BinaryOperator::Ampersand,
//             BinaryOp::BitwiseOr => BinaryOperator::Pipe,
//             BinaryOp::BitwiseXor => BinaryOperator::Caret,
//             BinaryOp::LeftShift => BinaryOperator::ShiftLeft,
//             BinaryOp::RightShift => BinaryOperator::ShiftRight,
//             BinaryOp::LogicalAnd => BinaryOperator::LogicalAnd,
//             BinaryOp::LogicalOr => BinaryOperator::LogicalOr,
//             BinaryOp::Equal => BinaryOperator::Equal,
//             BinaryOp::NotEqual => BinaryOperator::NotEqual,
//             BinaryOp::GreaterThan => BinaryOperator::GreaterThan,
//             BinaryOp::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
//             BinaryOp::LessThan => BinaryOperator::LessThan,
//             BinaryOp::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
//             BinaryOp::Assignment => BinaryOperator::Assign,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub enum Val {
//     Identifier(String),
//     Constant(i32),
// }

// impl std::fmt::Display for Val {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Val::Constant(n) => write!(f, "{}", n),
//             Val::Identifier(s) => write!(f, "{}", s),
//         }
//     }
// }


// #[derive(Clone, Debug)]
// pub enum Instruction {
//     Return(Val),
//     Unary { operator: UnaryOperator, src: Val, dst: Val },
//     Binary { operator: BinaryOperator, src1: Val, src2: Val, dst: Val },
//     Copy { src: Val, dst: Val },
//     Jump { label: Val },
//     JumpIfZero { src: Val, label: Val },
//     JumpIfNotZero { src: Val, label: Val },
//     Label { label: Val },
// }

// #[derive(Clone, Debug)]
// pub struct Function {
//     pub identifier: String,
//     pub body: Vec<Instruction>,
// }

// #[derive(Clone, Debug)]
// pub struct Program {
//     pub function: Function,
// }

// impl Factor {
//     fn generate_tac(&self, body: &mut Vec<Instruction>) -> Val {
//         match self {
//             Factor::Int(value) => Val::Constant(*value),
//             Factor::Unary(op, exp) => {
//                 let val = exp.generate_tac(body);
//                 let dst = Val::Identifier(format!("tmp.{}", body.len()));
//                 let instruction = Instruction::Unary {
//                     operator: UnaryOperator::from(op),
//                     src: val,
//                     dst: dst.clone(),
//                 };
//                 body.push(instruction);
//                 dst
//             }
//             Factor::Exp(exp) => exp.generate_tac(body),
//         }
//     }
// }

// impl Exp {
//     fn generate_tac(&self, body: &mut Vec<Instruction>) -> Val {
//         match self {
//             Exp::Factor(factor) => factor.generate_tac(body),
//             Exp::Binary(left, op, right) => {
//                 if op == &BinaryOp::LogicalAnd {
//                     let left_val = left.generate_tac(body);
//                     let dst = Val::Identifier(format!("tmp.{}", body.len()));
//                     let label = Val::Identifier(format!("label.{}", body.len()));
                    
//                     // Convert left value to boolean (0 or 1)
//                     let bool_dst = Val::Identifier(format!("tmp.{}", body.len() + 1));
//                     body.push(Instruction::Binary {
//                         operator: BinaryOperator::NotEqual,
//                         src1: left_val.clone(),
//                         src2: Val::Constant(0),
//                         dst: bool_dst.clone(),
//                     });
                    
//                     // Copy boolean result to dst
//                     body.push(Instruction::Copy {
//                         src: bool_dst.clone(),
//                         dst: dst.clone(),
//                     });
    
//                     // Short circuit if false (0)
//                     body.push(Instruction::JumpIfZero {
//                         src: bool_dst,
//                         label: label.clone(),
//                     });
                    
//                     // Evaluate right side if left was true
//                     let right_val = right.generate_tac(body);
                    
//                     // Convert right value to boolean and store in dst
//                     body.push(Instruction::Binary {
//                         operator: BinaryOperator::NotEqual,
//                         src1: right_val,
//                         src2: Val::Constant(0),
//                         dst: dst.clone(),
//                     });
    
//                     // Place the label for short-circuit
//                     body.push(Instruction::Label {
//                         label: label,
//                     });
    
//                     dst
//                 } else if op == &BinaryOp::LogicalOr {
//                     let left_val = left.generate_tac(body);
//                     let dst = Val::Identifier(format!("tmp.{}", body.len()));
//                     let label = Val::Identifier(format!("label.{}", body.len()));
                    
//                     // Convert left value to boolean (0 or 1)
//                     let bool_dst = Val::Identifier(format!("tmp.{}", body.len() + 1));
//                     body.push(Instruction::Binary {
//                         operator: BinaryOperator::NotEqual,
//                         src1: left_val.clone(),
//                         src2: Val::Constant(0),
//                         dst: bool_dst.clone(),
//                     });
                    
//                     // Copy boolean result to dst
//                     body.push(Instruction::Copy {
//                         src: bool_dst.clone(),
//                         dst: dst.clone(),
//                     });
    
//                     // Short circuit if true (1)
//                     body.push(Instruction::JumpIfNotZero {
//                         src: bool_dst,
//                         label: label.clone(),
//                     });
                    
//                     // Evaluate right side if left was false
//                     let right_val = right.generate_tac(body);
                    
//                     // Convert right value to boolean and store in dst
//                     body.push(Instruction::Binary {
//                         operator: BinaryOperator::NotEqual,
//                         src1: right_val,
//                         src2: Val::Constant(0),
//                         dst: dst.clone(),
//                     });
    
//                     // Place the label for short-circuit
//                     body.push(Instruction::Label {
//                         label: label,
//                     });
    
//                     dst
//                 } else {
//                     let left_val = left.generate_tac(body);
//                     let right_val = right.generate_tac(body);
//                     let dst = Val::Identifier(format!("tmp.{}", body.len()));
//                     body.push(Instruction::Binary {
//                         operator: BinaryOperator::from(op),
//                         src1: left_val,
//                         src2: right_val,
//                         dst: dst.clone(),
//                     });
//                     dst
//                 }
//             }
//         }
//     }
// }

// impl Statement {
//     fn generate_tac(&self, body: &mut Vec<Instruction>) {
//         match self {
//             Statement::Return(exp) => {
//                 let val = exp.generate_tac(body);
//                 body.push(Instruction::Return(val));
//             }
//         }
//     }
// }

// impl FunctionDeclaration {
//     pub fn generate_tac(&self) -> Function {
//         let mut body = Vec::new();
//         match self {
//             FunctionDeclaration::Function(identifier, statement) => {
//                 statement.generate_tac(&mut body);
//                 Function {
//                     identifier: identifier.clone(),
//                     body,
//                 }
//             }
//         }
//     }
// }

// impl ParserProgram {
//     pub fn generate_tac(&self) -> Program {
//         match self {
//             ParserProgram::Program(func_decl) => {
//                 let function = func_decl.generate_tac();
//                 Program { function }
//             }
//         }
//     }
// }

// pub fn generate_tac(program: ParserProgram) -> Program {
//     program.generate_tac()
// }