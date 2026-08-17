// Run with: cargo run
// Run tests with: cargo test
use lalrpop_util::lalrpop_mod;

// The lalrpop_mod! macro will include the generated parser code from OUT_DIR.
// It expands to a module declaration similar to this:
// pub mod use_location {
//    include!(concat!(env!("OUT_DIR"), "/use_location.rs"));
// }
// See target/debug/build/calculator-<hash>>/out/use_location.rs
lalrpop_mod!(pub location); // synthesized by LALRPOP
use crate::location::TermParser;

#[test]
fn use_location() {
    assert!(TermParser::new().parse("22").is_ok());
    assert!(TermParser::new().parse("(22)").is_ok());
    assert!(TermParser::new().parse("((((22))))").is_ok());
    assert!(TermParser::new().parse("((22)").is_err());
}

fn main() {
    let parser = TermParser::new();

    println!("22 =at pos=> {}", parser.parse("22").unwrap());
    println!("(22.5) =at pos => {}", parser.parse("(22.5)").unwrap());
    println!("((((22)))) =at pos => {}", parser.parse("((((22))))").unwrap());
    println!("Error example: ((22) {}", parser.parse("((22)").unwrap_err());
}