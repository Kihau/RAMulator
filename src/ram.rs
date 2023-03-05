use crate::{Instruction, OpType, OpCode};

/// Data that is held by a register
pub type RegisterData = i32;

/// Random Access Machine 
///
/// Responsible for executing RAM instructions, holds current state of the machine and its data 
#[derive(Default, Debug)]
pub struct RAM {
    /// State of the machine, set to `true` when the `HALT` is reached or the machine runs
    /// out of instructions to execute
    finished: bool,
    /// Stack of instructions that get executed by the machine
    instruction_stack: Vec<Instruction>,
    /// Points to the current instruction from the instruction stack
    instruction_pointer: usize,
    /// Registers that store data of the machine
    registers: Vec<RegisterData>,
    /// Currently loaded register. This register changes when the `LOAD` instruction gets executed
    loaded_reg: usize,
}

impl RAM {
    /// Creates a new virtual machine
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instruction_stack = instructions;
    }

    fn get_register_data(&mut self, idx: usize) -> RegisterData {
        if idx >= self.registers.len() {
            self.registers.resize(idx + 1, 0);
        }
        self.registers[idx]
    }

    fn get_readregister_data(&mut self, idx: usize) -> RegisterData {
        let reg_data = self.get_register_data(idx);
        self.get_register_data(reg_data as usize)
    }

    fn set_register_data(&mut self, idx: usize, data: RegisterData) {
        if idx >= self.registers.len() {
            self.registers.resize(idx + 1, 0);
        }
        self.registers[idx] = data;
    }

    fn get_instruction_data(&mut self, inst: &Instruction) -> i32 {
        match inst.op_type {
            OpType::Register => self.get_register_data(inst.op_value as usize),
            OpType::Value => inst.op_value,
            OpType::ReadReg => self.get_readregister_data(inst.op_value as usize),
            // TODO: This should be just unreachable
            OpType::NoValue => panic!("Instruction requires an argument"),
        }
    }
    pub fn print_instruction_stack(&self) {
        println!("---- INSTRUCTION STACK ----");
        for inst in &self.instruction_stack {
            println!("{inst}");
        }
        println!("---------------------------");
    }

    // TODO: Put some code as an implementation function for the Instruction structure
    //
    // TODO: All panics in this structure should be ignored. Validity check should be done on the
    // parsing step
    //
    /// Executes instruction under the instruction pointer and the returns it.
    pub fn execute_next_instruction(&mut self) -> Option<Instruction> {
        let inst_idx = self.instruction_pointer;
        if inst_idx == self.instruction_stack.len() || self.finished {
            self.finished = true;
            return None
        }

        let inst = self.instruction_stack[inst_idx].clone();
        self.instruction_pointer += 1;

        match inst.op_code {
            OpCode::LOAD => {
                self.loaded_reg = match inst.op_type {
                    OpType::Register => inst.op_value as usize,
                    OpType::ReadReg => self.get_register_data(inst.op_value as usize) as usize,
                    OpType::Value | OpType::NoValue => panic!("Instruction LOAD requires a register"),
                }
            }
            OpCode::STORE => {
                let data = self.get_instruction_data(&inst);
                self.set_register_data(self.loaded_reg, data);
                // let data = self.get_register_data(self.loaded_reg);
                // self.set_register_data(inst.op_value as usize, data);
            }
            OpCode::ADD => {
                let data = self.get_instruction_data(&inst);
                let loaded_data = self.get_register_data(self.loaded_reg);
                self.set_register_data(self.loaded_reg, loaded_data + data);
            }
            OpCode::SUB => {
                let data = self.get_instruction_data(&inst);
                let loaded_data = self.get_register_data(self.loaded_reg);
                self.set_register_data(self.loaded_reg, loaded_data - data);
            }
            OpCode::MULT => {
                let data = self.get_instruction_data(&inst);
                let loaded_data = self.get_register_data(self.loaded_reg);
                self.set_register_data(self.loaded_reg, loaded_data * data);
            }
            OpCode::DIV => {
                let data = self.get_instruction_data(&inst);
                let loaded_data = self.get_register_data(self.loaded_reg);
                self.set_register_data(self.loaded_reg, loaded_data / data);
            }
            OpCode::READ => {
                // TODO: Error handling
                let mut buffer = String::new();
                let _ = std::io::stdin().read_line(&mut buffer);
                let Ok(data) = buffer.trim().parse::<i32>() else {
                    eprintln!("ERROR: Incorrect READ data: Input argument must be a 32 bit integer");
                    return None;
                };

                let register = match inst.op_type {
                    OpType::Register => inst.op_value as usize,
                    OpType::ReadReg => self.get_register_data(inst.op_value as usize) as usize,
                    OpType::NoValue | OpType::Value => panic!("Instruction READ requires a register"),
                };
                self.set_register_data(register, data);
            }
            OpCode::WRITE => {
                let data = self.get_instruction_data(&inst);
                println!("{data}");
            }
            OpCode::JUMP => {
                let index = self.get_instruction_data(&inst);
                self.instruction_pointer = index as usize;
            }
            OpCode::JGTZ => {
                let loaded_data = self.get_register_data(self.loaded_reg);
                if loaded_data > 0 {
                    let index = self.get_instruction_data(&inst);
                    self.instruction_pointer = index as usize;
                }
            }
            OpCode::JZERO => {
                let loaded_data = self.get_register_data(self.loaded_reg);
                if loaded_data == 0 {
                    let index = self.get_instruction_data(&inst);
                    self.instruction_pointer = index as usize;
                }
            }
            OpCode::HALT => self.finished = true,
        };
        Some(inst)
    }
}

