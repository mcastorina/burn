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
            return_type,
        } => check_fn(name, parameters, body, return_type),
        ast::Item::Import { .. } => panic!("Imports are not supported"),
    }
}

fn check_fn(
    _name: &String,
    parameters: &Vec<(String, ast::Type)>,
    _body: &Vec<ast::Stmt>,
    return_type: &Option<ast::Type>,
) {
    for (_, param_type) in parameters {
        check_type(param_type.clone());
    }
    // TODO: check body stmt
    if let Some(typ) = return_type {
        check_type(typ.clone());
    }
}

fn check_type(mut typ: ast::Type) {
    match typ.name.as_ref() {
        "bool" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
            if typ.generics.len() != 0 {
                panic!("Basic types cannot have generics");
            }
        }
        "option" => {
            if typ.generics.len() != 1 {
                panic!("Expected exactly one generic for `option` type");
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
        }
        t => panic!("Unrecognized type: {}", t),
    }
    for generic in typ.generics {
        check_type(generic);
    }
}
