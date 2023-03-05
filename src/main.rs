use RAMulator::{parser::Parser, ram::RAM};

fn main() {
    // let code = std::fs::read_to_string("ram/add_numbers.ram").unwrap();
    // let code = std::fs::read_to_string("ram/example-fucked.ram").unwrap();
    // let code = std::fs::read_to_string("ram/example.ram").unwrap();
    // let code = std::fs::read_to_string("ram/sequence_sum.ram").unwrap();
    let code = std::fs::read_to_string("ram/slide.ram").unwrap();


    let mut parser = Parser::default();
    let instructions = parser.parse_source(code);

    let mut ram = RAM::new();
    ram.load_instructions(instructions);

    ram.print_instruction_stack();

    while let Some(inst) =  ram.execute_next_instruction() {
        println!("Executed: {inst}");
    }
}
