use crate::tac::{Program as TacProgram, Function as TacFunction, Instruction as TacInstruction, Val, UnaryOperator as TacUnaryOperator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Reg {
    AX,
    R10,
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
pub enum Instruction {
    Mov(Operand, Operand),
    Unary(UnaryOperator, Operand),
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
                }
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
                        }
                        _ => {
                            new_instructions.push(instr.clone());
                        }
                    }
                }
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
                }
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

