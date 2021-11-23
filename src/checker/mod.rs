use crate::parser::ast;
use std::collections::{HashMap, HashSet};

// have: list of items
// need to:
//  * gather metadata about each item (function declaration)
//    * function name, parameters, and return parameters
//  * check body of function definitions
//  * check scoping
//    * try one pass
//    * accumulate variable names / types
//    * keep reference to parent scope
//    * keep type information after checking
//

pub struct Checker<'a> {
    items: &'a Vec<ast::Item>,
    fn_names: HashMap<String, FnInfo>,
}

struct FnInfo {
    input_params: Vec<ast::Type>,
    output_params: Vec<ast::Type>,
}

impl<'a> Checker<'a> {
    pub fn new(items: &'a Vec<ast::Item>) -> Self {
        Self {
            items: items,
            fn_names: HashMap::new(),
        }
    }
    pub fn check_all(&mut self) {
        // TODO:
        // - check type safety
        let mut set: HashSet<&str> = HashSet::new();
        for item in self.items {
            let ast::Item::Function { name, .. } = item;
            if set.contains(name.as_str()) {
                panic!("Redeclaration of function name \"{}\"", name);
            }
            set.insert(name);
            self.check(item);
        }
        if !set.contains("main") {
            panic!("main function not found");
        }
    }

    fn check(&mut self, item: &ast::Item) {
        match item {
            ast::Item::Function {
                name,
                parameters,
                body,
                return_params,
            } => self.check_fn(name, parameters, body, return_params),
        }
    }

    fn check_fn(
        &mut self,
        _name: &String,
        params: &Vec<(String, ast::Type)>,
        body: &Vec<ast::Stmt>,
        return_params: &Vec<(String, ast::Type)>,
    ) {
        let mut set = HashSet::new();
        for (name, param_type) in params.iter().chain(return_params.iter()) {
            self.check_type(param_type);
            if set.contains(name) {
                panic!("Redeclaration of parameter name \"{}\"", name);
            }
            set.insert(name);
        }
        self.check_block(body);
    }

    fn check_type(&mut self, typ: &ast::Type) {
        match typ.name.as_ref() {
            "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
                self.check_basic_type(typ);
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
                    self.check_type(&generic);
                }
            }
            t => panic!("Unrecognized type: {}", t),
        }
    }

    fn check_basic_type(&mut self, typ: &ast::Type) {
        match typ.name.as_ref() {
            "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
                if typ.generics.len() != 0 {
                    panic!("Basic types cannot have generics");
                }
            }
            t => panic!("Unrecognized basic type: {}", t),
        }
    }

    fn check_block(&mut self, block: &Vec<ast::Stmt>) {}
}
