// Run with: cargo run
// Run tests with: cargo test
use lalrpop_util::lalrpop_mod;

mod ast;

lalrpop_mod!(pub buildingast); // synthesized by LALRPOP
use crate::buildingast::ExprParser;

#[test]
fn calculator4() {
    let expr = buildingast::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{}", expr), "((22 * 44) + 66)");
}

fn main() {
    let parser = ExprParser::new();

    println!("3-2-1 = {}", parser.parse("3-2-1").unwrap());
    println!("2+3*5 = {}", parser.parse("2+3*5").unwrap());
    println!("(4-2)*2 = {}", parser.parse("(4-2)*2").unwrap());
    println!("Error example: ((22) = {}", parser.parse("((22)").unwrap_err());
}