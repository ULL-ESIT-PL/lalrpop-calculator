// Run with: cargo run
// Run tests with: cargo test
use lalrpop_util::lalrpop_mod;

// The lalrpop_mod! macro will include the generated parser code from OUT_DIR.
// It expands to a module declaration similar to this:
// pub mod calculator1 {
//    include!(concat!(env!("OUT_DIR"), "/calculator1.rs"));
// }
// See target/debug/build/calculator-<hash>>/out/calculator1.rs
lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
use crate::calculator1::TermParser;

#[test]
fn calculator1() {
    assert!(TermParser::new().parse("22").is_ok());
    assert!(TermParser::new().parse("(22)").is_ok());
    assert!(TermParser::new().parse("((((22))))").is_ok());
    assert!(TermParser::new().parse("((22)").is_err());
}

fn main() {
    let parser = TermParser::new();

    println!("22 = {}", parser.parse("22").unwrap());
    println!("(22.5) = {}", parser.parse("(22.5)").unwrap());
    println!("((((22)))) = {}", parser.parse("((((22))))").unwrap());
    println!("Error example: ((22) = {}", parser.parse("((22)").unwrap_err());
}