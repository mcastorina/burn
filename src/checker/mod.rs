use crate::parser::ast;
use std::collections::HashSet;

pub fn check_all(items: &Vec<ast::Item>) {
    // TODO:
    // - check type safety
    let mut set: HashSet<&str> = HashSet::new();
    for item in items {
        let ast::Item::Function { name, .. } = item;
        if set.contains(name.as_str()) {
            panic!("Redeclaration of function name \"{}\"", name);
        }
        set.insert(name);
        check(item);
    }
    if !set.contains("main") {
        panic!("main function not found");
    }
}

pub fn check(item: &ast::Item) {
    match item {
        ast::Item::Function {
            name,
            parameters,
            body,
            return_params,
        } => check_fn(name, parameters, body, return_params),
    }
}

fn check_fn(
    _name: &String,
    params: &Vec<(String, ast::Type)>,
    body: &Vec<ast::Stmt>,
    return_params: &Vec<(String, ast::Type)>,
) {
    let mut set = HashSet::new();
    for (name, param_type) in params.iter().chain(return_params.iter()) {
        check_type(param_type);
        if set.contains(name) {
            panic!("Redeclaration of parameter name \"{}\"", name);
        }
        set.insert(name);
    }
    check_block(body);
}

fn check_type(typ: &ast::Type) {
    match typ.name.as_ref() {
        "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
            check_basic_type(typ);
        }
        // TODO: decide if I want option types
        // "option" => {
        //     if typ.generics.len() != 1 {
        //         panic!("Expected exactly one generic for `option` type");
        //     }
        //     for generic in &typ.generics {
        //         check_type(&generic);
        //     }
        // }
        "stream" => {
            if typ.generics.len() != 1 {
                panic!("Expected exactly one generic for `stream` type");
            }
            for generic in &typ.generics {
                check_type(&generic);
            }
        }
        t => panic!("Unrecognized type: {}", t),
    }
}

fn check_basic_type(typ: &ast::Type) {
    match typ.name.as_ref() {
        "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
            if typ.generics.len() != 0 {
                panic!("Basic types cannot have generics");
            }
        }
        t => panic!("Unrecognized basic type: {}", t),
    }
}

fn check_block(block: &Vec<ast::Stmt>) {}
