use std::collections::HashMap;

use crate::{Instruction, OpCode, OpType};

/// Responsible for parsing RAM source into instructions.
///
/// Stores required information to create *correct* RAM instruction code
#[derive(Default)]
pub struct Parser {
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

    pub fn parse_source(&mut self, source: String) -> Vec<Instruction> {
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

        instruction_stack
    }
}

