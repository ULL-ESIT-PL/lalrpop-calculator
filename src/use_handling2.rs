// Run with: cargo run
// Run tests with: cargo test
use lalrpop_util::lalrpop_mod;

// The lalrpop_mod! macro will include the generated parser code from OUT_DIR.
// It expands to a module declaration similar to this:
// pub mod handling2 {
//    include!(concat!(env!("OUT_DIR"), "/handling2.rs"));
// }
// See target/debug/build/calculator-<hash>>/out/handling2.rs
lalrpop_mod!(pub handling2); // synthesized by LALRPOP
use crate::handling2::ExprParser;

#[test]
fn handling2() {
    assert!(ExprParser::new().parse("3-2-1").is_ok());
    assert!(ExprParser::new().parse("2+3*5").is_ok());
    assert!(ExprParser::new().parse("(4-2)*2").is_ok());
    assert!(ExprParser::new().parse("((22)").is_err());
}

fn main() {
    let parser = ExprParser::new();

    println!("3-2-1 = {}", parser.parse("3-2-1").unwrap());
    println!("2+3*5 = {}", parser.parse("2+3*5").unwrap());
    println!("(4-2)*2 = {}", parser.parse("(4-2)*2").unwrap());
    println!("Error example: ((22) = {}", parser.parse("((22)").unwrap_err());
}