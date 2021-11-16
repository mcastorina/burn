pub mod checker;
pub mod lexer;
pub mod parser;

use std::fs;

fn compile(program: &str) {
    let mut parser = parser::Parser::new(program);
    let items = parser.file();
    let mut checker = checker::Checker::new(&items);
    checker.check_all();
}

pub fn compile_file(input_filename: &str) {
    let program = fs::read_to_string(input_filename).unwrap();
    compile(&program);
}
