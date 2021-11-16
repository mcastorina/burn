use burn::parser::Parser;
use burn::checker::Checker;

fn run_checker(input: &str) {
    let mut parser = Parser::new(input);
    let items = parser.file();
    let mut checker = Checker::new(&items);
    checker.check_all();
}

#[test]
fn check_fn_param_names_unique() {
    run_checker("fn main(foo u32, bar u32) -> (baz u32) {}");
}

#[test]
#[should_panic]
fn check_fn_param_names_not_unique() {
    run_checker("fn main(foo u32, bar u32) -> (foo u32) {}");
}

#[test]
fn check_fn_param_types_valid() {
    run_checker("fn main(foo stream<stream<u8>>, bar i32) -> (baz u64, buz stream<i64>) {}");
}

#[test]
#[should_panic]
fn check_fn_param_types_invalid() {
    run_checker("fn main(foo stream<u32>) -> (bar int) {}");
}


#[test]
fn check_fn_names_unique() {
    run_checker("fn main() {} fn foo() {}");
}

#[test]
#[should_panic]
fn check_fn_names_not_unique() {
    run_checker("fn main() {} fn main() {}");
}

#[test]
#[should_panic]
fn check_no_main() {
    run_checker("fn foo() {} fn bar() {}");
}

#[test]
fn check_fn_arity() {}
