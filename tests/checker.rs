use burn::checker;
use burn::parser::ast::Item;
use burn::parser::Parser;

fn parse_item(input: &str) -> Item {
    let mut parser = Parser::new(input);
    parser.item()
}

fn parse_items(input: &str) -> Vec<Item> {
    let mut parser = Parser::new(input);
    parser.file()
}

#[test]
fn check_fn_param_names_unique() {
    let item = parse_item("fn test(foo u32, bar u32) -> (baz u32) {}");
    // checker will panic on error
    checker::check(&item);
}

#[test]
#[should_panic]
fn check_fn_param_names_not_unique() {
    let item = parse_item("fn test(foo u32, bar u32) -> (foo u32) {}");
    // checker will panic on error
    checker::check(&item);
}

#[test]
fn check_fn_param_types_valid() {
    let item =
        parse_item("fn test(foo stream<stream<u8>>, bar i32) -> (baz u64, buz stream<i64>) {}");
    // checker will panic on error
    checker::check(&item);
}

#[test]
#[should_panic]
fn check_fn_param_types_invalid() {
    let item = parse_item("fn test(foo stream<u32>) -> (bar int) {}");
    // checker will panic on error
    checker::check(&item);
}

#[test]
fn check_fn_names_unique() {
    let items = parse_items("fn main() {} fn foo() {}");
    assert_eq!(items.len(), 2);
    // checker will panic on error
    checker::check_all(&items);
}

#[test]
#[should_panic]
fn check_fn_names_not_unique() {
    let items = parse_items("fn main() {} fn main() {}");
    // checker will panic on error
    checker::check_all(&items);
}

#[test]
#[should_panic]
fn check_no_main() {
    let items = parse_items("fn foo() {} fn bar() {}");
    // checker will panic on error
    checker::check_all(&items);
}

#[test]
fn check_fn_arity() {}
