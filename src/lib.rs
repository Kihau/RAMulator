pub mod ram;
pub mod parser;

/// Random Access Machine Opcodes
#[derive(Debug, Clone)]
pub enum OpCode {
    /// Loads data from specified register to the adder register
    LOAD = 0,
    /// Copy data from adder to the specified register 
    STORE = 1,
    /// Add value or data from specified register to the adder register
    ADD = 2,
    /// Subtract value from the adder register with value or data in specified register
    SUB = 3,
    /// Multiply data in the adder register with a value or data from specified register
    MULT = 4,
    /// Divide value of the adder register with value or data in specified register
    DIV = 5,
    /// Read data from input memory (here memory is stdin) and load it into specified register
    READ = 6,
    /// Write value or data from specified register to output memory (here memory is stdout)
    WRITE = 7,
    /// Jump to label (or value)
    JUMP = 8,
    /// Jump to label (or value) if value under the adder register is greater than zero
    JGTZ = 9,
    /// Jump to label (or value) if value under the adder register is zero
    JZERO = 10,
    /// End the code execution
    HALT = 11,
}

/// Type of the operand
#[derive(Debug, Clone)]
pub enum OpType {
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
pub struct Instruction {
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
