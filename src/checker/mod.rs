use crate::parser::ast;

pub fn check_all(items: &Vec<ast::Item>) {
    // TODO:
    // - check there is a main fn
    // - check type safety
    for item in items {
        check(item);
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
    parameters: &Vec<(String, ast::Type)>,
    _body: &Vec<ast::Stmt>,
    return_params: &Vec<(String, ast::Type)>,
) {
    for (_, param_type) in parameters {
        check_type(param_type.clone());
    }
    // TODO: check return params
    // TODO: check body stmt
}

fn check_type(mut typ: ast::Type) {
    match typ.name.as_ref() {
        "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
            check_basic_type(typ);
        }
        "option" => {
            if typ.generics.len() != 1 {
                panic!("Expected exactly one generic for `option` type");
            }
            for generic in typ.generics {
                check_type(generic);
            }
        }
        "stream" => {
            // stream with a generic defaults to stream<u8>
            if typ.generics.len() == 0 {
                typ.generics.push(ast::Type {
                    name: "u8".to_string(),
                    generics: Vec::new(),
                });
            }
            if typ.generics.len() != 1 {
                panic!("Expected exactly one generic for `stream` type");
            }
            for generic in typ.generics {
                check_basic_type(generic);
            }
        }
        t => panic!("Unrecognized type: {}", t),
    }
}

fn check_basic_type(typ: ast::Type) {
    match typ.name.as_ref() {
        "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
            if typ.generics.len() != 0 {
                panic!("Basic types cannot have generics");
            }
        }
        t => panic!("Unrecognized basic type: {}", t),
    }
}
