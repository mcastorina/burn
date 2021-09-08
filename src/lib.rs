pub mod checker;
pub mod gen;
pub mod lexer;
pub mod parser;

use std::fs;

fn compile_to_c(program: &str) -> String {
    let mut parser = parser::Parser::new(program);
    let items = parser.file();
    checker::check_all(&items);
    gen::generate_c(items)
}

fn compile_file_to_c(input_filename: &str, output_filename: &str) {
    let program = fs::read_to_string(input_filename).unwrap();
    fs::write(output_filename, compile_to_c(&program)).unwrap();
}
