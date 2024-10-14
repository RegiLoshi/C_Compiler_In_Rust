use crate::tac::{Program as TacProgram, Function as TacFunction, Instruction as TacInstruction, Val, UnaryOperator as TacUnaryOperator, BinaryOperator as TacBinaryOperator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Reg {
    AX,
    DX,
    R10,
    R11,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(i32),
    Register(Reg),
    Pseudo(String),
    Stack(i32),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(Operand, Operand),
    Unary(UnaryOperator, Operand),
    Binary(BinaryOperator, Operand, Operand),
    Idiv(Operand),
    Cdq, //sign extension
    AllocateStack(i32),
    Ret,
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Program {
    function: Function,
}

impl From<TacUnaryOperator> for UnaryOperator {
    fn from(op: TacUnaryOperator) -> Self {
        match op {
            TacUnaryOperator::Negate => UnaryOperator::Neg,
            TacUnaryOperator::Complement => UnaryOperator::Not,
        }
    }
}

impl From<Val> for Operand {
    fn from(val: Val) -> Self {
        match val {
            Val::Constant(int) => Operand::Imm(int),
            Val::Identifier(id) => Operand::Pseudo(id),
        }
    }
}

impl TacInstruction {
    fn to_assembly_instructions(&self) -> Vec<Instruction> {
        match self {
            TacInstruction::Return(val) => vec![
                Instruction::Mov(Operand::from(val.clone()), Operand::Register(Reg::AX)),
                Instruction::Ret,
            ],
            TacInstruction::Unary { operator, src, dst } => vec![
                Instruction::Mov(Operand::from(src.clone()), Operand::from(dst.clone())),
                Instruction::Unary(UnaryOperator::from(operator.clone()), Operand::from(dst.clone())),
            ],
            TacInstruction::Binary { operator, src1, src2, dst } => {
                match operator {
                    // Handling the division operator
                    TacBinaryOperator::Divide => vec![
                        Instruction::Mov(Operand::from(src1.clone()), Operand::Register(Reg::AX)),
                        Instruction::Cdq,
                        Instruction::Idiv(Operand::from(src2.clone())),
                        Instruction::Mov(Operand::Register(Reg::AX), Operand::from(dst.clone())),
                    ],

                    // Handling the modulo operator
                    TacBinaryOperator::Modulo => vec![
                        Instruction::Mov(Operand::from(src1.clone()), Operand::Register(Reg::AX)),
                        Instruction::Cdq,
                        Instruction::Idiv(Operand::from(src2.clone())),
                        Instruction::Mov(Operand::Register(Reg::DX), Operand::from(dst.clone())),
                    ],

                    // Handling other binary operators
                    _ => vec![
                        Instruction::Mov(Operand::from(src1.clone()), Operand::from(dst.clone())),
                        Instruction::Binary(
                            match operator {
                                TacBinaryOperator::Add => BinaryOperator::Add,
                                TacBinaryOperator::Subtract => BinaryOperator::Sub,
                                TacBinaryOperator::Multiply => BinaryOperator::Mul,
                                _ => panic!("Invalid operator"),
                            },
                            Operand::from(src2.clone()),
                            Operand::from(dst.clone()),
                        ),
                    ],
                }
            }
        }
    }
}

impl TacFunction {
    fn to_assembly_function(&self) -> Function {
        let instructions: Vec<Instruction> = self.body.iter()
            .flat_map(|instr| instr.to_assembly_instructions())
            .collect();
        
        Function {
            name: self.identifier.clone(),
            instructions,
        }
    }
}

impl TacProgram {
    pub fn to_assembly_program(&self) -> Program {
        let function = self.function.to_assembly_function();
        Program { function }
    }
}

impl Operand {
    pub fn to_assembly_file(&self) -> String {
        match self {
            Operand::Imm(int) => format!("${}", int),
            Operand::Register(reg) => match reg {
                Reg::AX => "%eax".to_string(),
                Reg::R10 => "%r10d".to_string(),
                Reg::R11 => "%r11d".to_string(),
                Reg::DX => "%edx".to_string(),
            },
            Operand::Pseudo(id) => id.clone(),
            Operand::Stack(offset) => format!("{}(%rbp)", offset),
        }
    }
}

impl Function {
    pub fn replace_pseudo(&mut self) -> i32 {
        let mut pseudo_map = HashMap::new();
        let mut new_instructions = Vec::new();
        let mut counter = -4;

        for instr in self.instructions.iter() {
            match instr {
                Instruction::Mov(src, dst) => {
                    let new_src = Self::replace_operand(src, &mut pseudo_map, &mut counter);
                    let new_dst = Self::replace_operand(dst, &mut pseudo_map, &mut counter);
                    new_instructions.push(Instruction::Mov(new_src, new_dst));
                }
                Instruction::Unary(op, dst) => {
                    let new_dst = Self::replace_operand(dst, &mut pseudo_map, &mut counter);
                    new_instructions.push(Instruction::Unary(op.clone(), new_dst));
                },
                Instruction::Binary(op, src, dst) => {
                    let new_src = Self::replace_operand(src, &mut pseudo_map, &mut counter);
                    let new_dst = Self::replace_operand(dst, &mut pseudo_map, &mut counter);
                    new_instructions.push(Instruction::Binary(op.clone(), new_src, new_dst));
                },
                Instruction::Idiv(op) => {
                    let new_op = Self::replace_operand(op, &mut pseudo_map, &mut counter);
                    new_instructions.push(Instruction::Idiv(new_op));
                },
                _ => new_instructions.push(instr.clone()),
            }
        }

        self.instructions = new_instructions;
        -counter 
    }

    fn replace_operand(
        operand: &Operand,
        pseudo_map: &mut HashMap<String, Operand>,
        counter: &mut i32
    ) -> Operand {
        match operand {
            Operand::Pseudo(id) => {
                pseudo_map.entry(id.clone()).or_insert_with(|| {
                    let new_op = Operand::Stack(*counter);
                    *counter -= 4;
                    new_op
                }).clone()
            }
            _ => operand.clone(),
        }
    }

    pub fn fixMov(&mut self, stackSize: i32) {
        let mut new_instructions = Vec::new();
        for instr in self.instructions.iter() {
            match instr {
                Instruction::Mov(src, dst) => {
                    match (src, dst) {
                        (Operand::Stack(_), Operand::Stack(_)) => {
                            new_instructions.push(Instruction::Mov(src.clone(), Operand::Register(Reg::R10)));
                            new_instructions.push(Instruction::Mov(Operand::Register(Reg::R10), dst.clone()));
                        },
                        _ => {
                            new_instructions.push(instr.clone());
                        }
                    }
                },
                Instruction::Binary(op, src, dst) => {
                    match (op, src, dst) {
                        (BinaryOperator::Add, Operand::Stack(_), Operand::Stack(_)) |
                        (BinaryOperator::Sub, Operand::Stack(_), Operand::Stack(_)) => {
                            new_instructions.push(Instruction::Mov(src.clone(), Operand::Register(Reg::R10)));
                            new_instructions.push(Instruction::Binary(op.clone(), Operand::Register(Reg::R10), dst.clone()));
                        },
                        (BinaryOperator::Mul, Operand::Imm(_), dst @ Operand::Stack(_)) => {
                            new_instructions.push(Instruction::Mov(dst.clone(), Operand::Register(Reg::R11)));
                            new_instructions.push(Instruction::Binary(BinaryOperator::Mul, src.clone(), Operand::Register(Reg::R11)));
                            new_instructions.push(Instruction::Mov(Operand::Register(Reg::R11), dst.clone()));
                        },
                        (BinaryOperator::Mul, src @ Operand::Stack(_), dst @ Operand::Stack(_)) => {
                            new_instructions.push(Instruction::Mov(src.clone(), Operand::Register(Reg::R10)));
                            new_instructions.push(Instruction::Mov(dst.clone(), Operand::Register(Reg::R11)));
                            new_instructions.push(Instruction::Binary(BinaryOperator::Mul, Operand::Register(Reg::R10), Operand::Register(Reg::R11)));
                            new_instructions.push(Instruction::Mov(Operand::Register(Reg::R11), dst.clone()));
                        },
                        _ => {
                            new_instructions.push(instr.clone());
                        }
                    }
                },
                Instruction::Idiv(op) => {
                    match op {
                        Operand::Imm(_) => {
                            new_instructions.push(Instruction::Mov(op.clone(), Operand::Register(Reg::R10)));
                            new_instructions.push(Instruction::Idiv(Operand::Register(Reg::R10)));
                        },
                        _ => {
                            new_instructions.push(instr.clone());
                        }
                    }
                },
                _ => {
                    new_instructions.push(instr.clone());
                }
            }
        }
        self.instructions = new_instructions;
        self.instructions.insert(0, Instruction::AllocateStack(stackSize));
    }

    pub fn to_assembly_file(mut self, result: &mut String) {
        result.push_str(&format!(".globl _{}\n", self.name));
        result.push_str(&format!("_{}:\n", self.name));
        result.push_str("pushq %rbp\n");
        result.push_str("movq %rsp, %rbp\n");
        for instr in self.instructions.iter() {
            match instr {
                Instruction::Mov(src, dst) => {
                    result.push_str(&format!("movl {}, {}\n", src.to_assembly_file(), dst.to_assembly_file()));
                }
                Instruction::Unary(op, dst) => {
                    match op {
                        UnaryOperator::Neg => {
                            result.push_str(&format!("negl {}\n", dst.to_assembly_file()))
                        }
                        UnaryOperator::Not => {
                            result.push_str(&format!("notl {}\n", dst.to_assembly_file()))
                        }
                    }
                }
                Instruction::AllocateStack(size) => {
                    result.push_str(&format!("subq ${}, %rsp\n", size));
                }
                Instruction::Ret => {
                    result.push_str("movq %rbp, %rsp\n");
                    result.push_str("popq %rbp\n");
                    result.push_str("ret\n");
                },
                Instruction::Binary(op, src, dst) => {
                    match op {
                        BinaryOperator::Add => {
                            result.push_str(&format!("addl {}, {}\n", src.to_assembly_file(), dst.to_assembly_file()));
                        }
                        BinaryOperator::Sub => {
                            result.push_str(&format!("subl {}, {}\n", src.to_assembly_file(), dst.to_assembly_file()));
                        }
                        BinaryOperator::Mul => {
                            result.push_str(&format!("imull {}, {}\n", src.to_assembly_file(), dst.to_assembly_file()));
                        }
                    }
                },
                Instruction::Idiv(op) => {
                    result.push_str(&format!("idivl {}\n", op.to_assembly_file()));
                },
                Instruction::Cdq => {
                    result.push_str("cdq\n");
                },
            }
        }
    }
}

impl Program {
    pub fn applyFixes(&mut self) {
        let stack_size = self.function.replace_pseudo();
        self.function.fixMov(stack_size);
    }

    pub fn to_assembly_file(&self) -> String {
        let mut result = String::new();
        self.function.clone().to_assembly_file(&mut result);
        result
    }
}

pub fn generate_assembly_AST(program: TacProgram) -> Program {
    program.to_assembly_program()
}

