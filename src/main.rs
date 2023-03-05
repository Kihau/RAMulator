/// Random Access Machine Opcodes
#[derive(Debug, Clone)]
enum OpCode {
    /// Changes the currently loaded register to specified register
    LOAD = 0,
    /// Copy value or register data from specified register to currently loaded register
    STORE = 1,
    /// Add value from specified register to currently loaded register
    ADD = 2,
    /// Subtract value from currently loaded register with value or data in specified register and
    /// put it into currently loaded register
    SUB = 3,
    /// Multiply value from currently loaded register with value or data in specified register and
    /// put it into currently loaded register
    MULT = 4,
    /// Divide value of the currently loaded register with value or data in specified register and
    /// put it into currently loaded register
    DIV = 5,
    /// Read data from input memory (here memory is stdin) and load it to the specified register
    READ = 6,
    /// Write value or data from specified register to output memory (here memory is stdout)
    WRITE = 7,
    /// Jump to label (or value)
    JUMP = 8,
    /// Jump to label (or value) if value under loaded register is greater than zero
    JGTZ = 9,
    /// Jump to label (or value) if value under loaded register is zero
    JZERO = 10,
    /// End the code execution
    HALT = 11,
}

/// Type of the operand
#[derive(Debug, Clone)]
enum OpType {
    /// Use the register
    ///
    /// Example: `ADD 1`
    Register = 0,
    /// Use the value
    ///
    /// Example: `ADD =1`
    Value = 1,
    /// Read the value under the register and use it as register
    ///
    /// Example: `ADD *1`
    ReadReg = 2,
    /// No value associated with the OpCode
    ///
    /// Example: `HALT`
    NoValue = 3,
}

// enum OpValue {
//     // ADD 1
//     Register(i32),
//     // ADD =1
//     Value(i32),
//     // ADD *1
//     ReadReg(i32),
// }

/// Represents a single instruction in the RAM code.
///
/// For example `ADD =12` translates to: 
/// ```rust
/// Instruction {
///     op_code: OpCode::ADD,
///     op_type: OpType::Value,
///     op_value: 12,
/// };
/// ```
#[derive(Debug, Clone)]
struct Instruction {
    op_code: OpCode,
    op_type: OpType,
    op_value: i32,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op_type {
            OpType::Register => write!(f, "{:?}\t {}", self.op_code, self.op_value),
            OpType::Value => write!(f, "{:?}\t={}", self.op_code, self.op_value),
            OpType::ReadReg => write!(f, "{:?}\t*{}", self.op_code, self.op_value),
            OpType::NoValue => write!(f, "{:?}", self.op_code),
        }
    }
}

/// Data that is held by a register
pub type RegisterData = i32;

/// Random Access Machine - TODO: add more info
#[derive(Default, Debug)]
struct RAM {
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

// TODO: Add a way to verify wheter an instruction is correct or not. 
impl RAM {
    /// Creates a new machine
    fn new() -> Self {
        Self::default()
    }

    fn load_instructions(&mut self, source: String) {
        // TODO: Parses/Lexer data
        let mut missing_labels = Vec::<(String, usize)>::new();
        let mut label_map = std::collections::HashMap::<String, usize>::new();
        let mut cursor = 0;

        // TODO: Parser
        //       The Parser struct should implement a `read_instruction` function that reads an
        //       instruction line from specified input stream (file, stdin, string, etc) and then
        //       returns it
        for line in source.lines() {
            let data: Vec<&str> = line.split_whitespace().collect();
            // println!("{data:?}");

            if data.is_empty() {
                continue;
            }

            let mut opcode_string = data[0].to_string();

            // TODO: Inline comments - comments in the same line as instructions
            //
            // The ; sign at the start of the string is considered to be a comment in my
            // implementation
            if opcode_string.starts_with(';') {
                continue;
            }

            // TODO: Inline jumps 
            //       Jump labels could be inlined and placed in the same line as the RAM 
            //       instruction, something like this:
            //       `my_label: READ 1`
            // Strings that end with the : are considered to be jump labels
            if opcode_string.ends_with(':') {
                opcode_string.pop();
                if label_map.contains_key(&opcode_string) {
                    panic!("Having two labels with the same name is not allowed");
                }
                label_map.insert(opcode_string, cursor);
                continue;
            }

            // TODO: This could be case insensitive
            let op_code = match opcode_string.as_str() {
                "LOAD"  => OpCode::LOAD,
                "STORE" => OpCode::STORE,
                "ADD"   => OpCode::ADD,
                "SUB"   => OpCode::SUB,
                "MULT"  => OpCode::MULT,
                "DIV"   => OpCode::DIV,
                "READ"  => OpCode::READ,
                "WRITE" => OpCode::WRITE,
                "JUMP"  => OpCode::JUMP,
                "JGTZ"  => OpCode::JGTZ,
                "JZERO" => OpCode::JZERO,
                "HALT"  => OpCode::HALT,
                _       => panic!("Given instruction does not exist.")
            };

            // OpCode has no second argument
            let Some(value) = data.get(1) else {
                let inst = Instruction {
                    op_code,
                    op_type: OpType::NoValue,
                    op_value: 0,
                };
                self.instruction_stack.push(inst);

                cursor += 1;
                continue;
            };

            let mut op_type;
            let mut value_chars = value.chars();
            if value.starts_with('*') {
                value_chars.next();
                op_type = OpType::ReadReg;
            } else if value.starts_with('=') {
                value_chars.next();
                op_type = OpType::Value;
            } else {
                op_type = OpType::Register;
            }

            // dbg!(value_chars.as_str());
            let op_value = if let Ok(value) = value_chars.as_str().parse::<i32>() {
                value
            } else {
                op_type = OpType::Value;
                missing_labels.push((value.to_string(), cursor));
                -1
            };

            let inst = Instruction {
                op_code, op_type, op_value,
            };
            self.instruction_stack.push(inst);

            cursor += 1;
        }

        for label in missing_labels {
            let value = label_map[&label.0];
            self.instruction_stack[label.1].op_value = value as i32;
        }
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
            OpType::Value => inst.op_value,
            OpType::Register => self.get_register_data(inst.op_value as usize),
            OpType::ReadReg => self.get_readregister_data(inst.op_value as usize),
            OpType::NoValue => panic!("Instruction requires an argument"),
        }
    }

    // TODO: Put some code as an implmentation function for the Instruction structure
    /// Executes instruction under the instruction pointer and the returns it.
    fn execute_next_instruction(&mut self) -> Option<Instruction> {
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
                let data = buffer.trim().parse::<i32>().unwrap();

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

fn main() {
    // let code = std::fs::read_to_string("ram/add_numbers.ram").unwrap();
    // let code = std::fs::read_to_string("ram/example-fucked.ram").unwrap();
    let code = std::fs::read_to_string("ram/sequence_sum.ram").unwrap();
    let mut ram = RAM::new();
    ram.load_instructions(code);

    println!("--------");
    for inst in &ram.instruction_stack {
        println!("{inst}");
    }

    while let Some(inst) =  ram.execute_next_instruction() {
        println!("Executed: {inst}");
        // println!("Data state: {:?}", ram.registers);
    }
}
