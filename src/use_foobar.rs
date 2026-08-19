// Run with: cargo run --bin foobar
// Run tests with: cargo test --bin foobar
use lalrpop_util::lalrpop_mod;

// The lalrpop_mod! macro will include the generated parser code from OUT_DIR.
lalrpop_mod!(pub foobar); // synthesized by LALRPOP
use crate::foobar::FooBarParser;

#[test]
fn use_foobar() {
    assert!(FooBarParser::new().parse("ab").is_ok());
    assert!(FooBarParser::new().parse("cab").is_ok());
}

fn main() {
    let parser = FooBarParser::new(); // Creating a closure to avoid repeating the parser creation for each parse call
    let parse = |input| parser.parse(input);

    println!("Input ab => {}", parse("ab").unwrap());
    println!("Input cab => {}", parse("cab").unwrap());
}