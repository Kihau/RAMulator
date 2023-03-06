use RAMulator::{parser::Parser, ram::RAM, new_parser::NewParser, ui::run_app};

fn main() {
    let _ = run_app();

    // let code = std::fs::read_to_string("ram/add_numbers.ram").unwrap();
    // let code = std::fs::read_to_string("ram/example-fucked.ram").unwrap();
    // let code = std::fs::read_to_string("ram/example.ram").unwrap();
    // let code = std::fs::read_to_string("ram/sequence_sum.ram").unwrap();
    // let code = std::fs::read_to_string("ram/slide.ram").unwrap();
    let code = std::fs::read_to_string("ram/testing.ram").unwrap();

    let mut parser = Parser::default();

    let mut exp = NewParser::default();
    exp.parse_source(&code);
    dbg!(exp.tokens);

    let instructions = match parser.parse_source_new(code) {
        Ok(inst) => inst,
        Err(message) => {
            eprintln!("{message}");
            return;
        }
    };

    let mut ram = RAM::new();
    ram.load_instructions(instructions);

    ram.print_instruction_stack();

    while let Some(inst) =  ram.execute_next_instruction() {
        println!("Executed: {inst}");
    }
}
