use std::collections::HashMap;

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

/// Random Access Machine 
///
/// Responsible for executing RAM instructions, holds current state of machine and the data 
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

/// Responsible for parsing RAM source into instructions.
///
/// Stores required information to create *correct* RAM instruction code
#[derive(Default)]
struct Parser {
    /// Points at the current instruction 
    cursor: usize,
    /// Stores jump labels and corresponding instructions that the point to 
    missing_labels: Vec::<(String, usize)>,
    // (TODO: improve this description)
    /// Stores jump instruction positions that are missing the jump index (??????)
    label_map: HashMap::<String, usize>,
}

// TODO: More verbose error on parsing, and don't use the crappy panic
// TODO: Add a way to verify whether an instruction is correct or not. 
impl Parser {
    /// Parses an instruction and returns it if the parsing succeeded. On failure to function 
    /// returns None.
    fn parse_instruction(&mut self, line: &str) -> Option<Instruction> {
        let mut data = line.split_whitespace();

        let mut opcode_string = if let Some(opcode_str) = data.next() {
            opcode_str.to_string()
        } else {
            return None;
        };

        // The ; sign at the start of the string is considered to be a comment in my
        // implementation
        if opcode_string.starts_with(';') {
            return None;
        }

        // Strings that end with the : are considered to be jump labels
        while opcode_string.ends_with(':') {
            opcode_string.pop();
            if opcode_string.is_empty() {
                panic!("Lable cannot be an empty string");
            }

            if self.label_map.contains_key(&opcode_string) {
                panic!("Having two labels with the same name is not allowed");
            }
            self.label_map.insert(opcode_string, self.cursor);


            opcode_string = if let Some(opcode_str) = data.next() {
                opcode_str.to_string()
            } else {
                return None;
            };
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

        let string = data.next();
        let value = if string.is_some() && !string.unwrap().starts_with(';') {
            string.unwrap()
        } else {
            // OpCode has no second argument or the argument is a comment
            let inst = Instruction {
                op_code,
                op_type: OpType::NoValue,
                op_value: 0,
            };

            self.cursor += 1;
            return Some(inst);
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
            self.missing_labels.push((value.to_string(), self.cursor));
            -1
        };

        let inst = Instruction {
            op_code, op_type, op_value,
        };

        self.cursor += 1;
        Some(inst)
    }

    fn parse_source(&mut self, source: String) -> Vec<Instruction> {
        let mut instruction_stack = Vec::new();

        for line in source.lines() {
            let instruction = self.parse_instruction(line);
            if let Some(inst) = instruction {
                instruction_stack.push(inst);
            }
        }

        // Filling the missing jump values
        for label in &self.missing_labels {
            let value = self.label_map[&label.0];
            instruction_stack[label.1].op_value = value as i32;
        }

        return instruction_stack;
    }
}

impl RAM {
    /// Creates a new virtual machine
    fn new() -> Self {
        Self::default()
    }

    fn load_instructions(&mut self, instructions: Vec<Instruction>) {
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

    // TODO: Put some code as an implementation function for the Instruction structure
    //
    // TODO: All panics in this structure should be ignored. Validity check should be done on the
    // parsing step
    //
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
    let code = std::fs::read_to_string("ram/example.ram").unwrap();
    // let code = std::fs::read_to_string("ram/sequence_sum.ram").unwrap();


    let mut parser = Parser::default();
    let instructions = parser.parse_source(code);

    let mut ram = RAM::new();
    ram.load_instructions(instructions);

    println!("--------");
    for inst in &ram.instruction_stack {
        println!("{inst}");
    }

    while let Some(inst) =  ram.execute_next_instruction() {
        println!("Executed: {inst}");
        // println!("Data state: {:?}", ram.registers);
    }
}
