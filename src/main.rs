// Run with: cargo run
// Run tests with: cargo test
use lalrpop_util::lalrpop_mod;

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