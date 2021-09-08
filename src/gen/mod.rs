use crate::parser::ast;

pub fn generate_c(items: Vec<ast::Item>) -> String {
    let mut output = String::new();
    for item in items {
        let item_out = match item {
            ast::Item::Function {
                name,
                parameters,
                body,
                return_type,
            } => gen_fn(name, parameters, body, return_type),
            ast::Item::Import { .. } => "".to_string(), // not supported
        };
        output.push_str(&item_out);
    }
    output
}

fn gen_fn(
    name: String,
    parameters: Vec<(String, ast::Type)>,
    body: Vec<ast::Stmt>,
    return_type: Option<ast::Type>,
) -> String {
    let ret_type = return_type.map(|t| t.name).unwrap_or("void".to_string());
    let mut output = format!("{} {}() {{", ret_type, name);
    output.push_str("}");

    output
}

#[test]
fn gen_fn_return_type() {
    let output = gen_fn(
        "main".to_string(),
        Vec::new(),
        Vec::new(),
        Some(ast::Type {
            name: "u8".to_string(),
            generics: Vec::new(),
        }),
    );

    assert_eq!(output, "u8 main() {}");

    let output = gen_fn("main".to_string(), Vec::new(), Vec::new(), None);

    assert_eq!(output, "void main() {}");
}
