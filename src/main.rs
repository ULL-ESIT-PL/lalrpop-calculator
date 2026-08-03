// Run with: cargo run
// Run tests with: cargo test
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

fn main() {
    println!("22 = {}", calculator1::TermParser::new().parse("22").unwrap());
    println!("(22) = {}", calculator1::TermParser::new().parse("(22)").unwrap());
    println!("((((22)))) = {}", calculator1::TermParser::new().parse("((((22))))").unwrap());
    println!("((22) = {}", calculator1::TermParser::new().parse("((22)").unwrap_err());
}