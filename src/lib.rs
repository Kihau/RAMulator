pub mod ram;
pub mod parser;

/// Random Access Machine Opcodes
#[derive(Debug, Clone)]
pub enum OpCode {
    /// Changes the currently loaded register to specified register
    LOAD = 0,
    // TODO: The logic should be flipped here I think
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
