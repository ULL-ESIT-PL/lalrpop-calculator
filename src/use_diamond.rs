// Run with: cargo run --bin diamond
// Run tests with: cargo test --bin diamond
use lalrpop_util::lalrpop_mod;

// The lalrpop_mod! macro will include the generated parser code from OUT_DIR.
lalrpop_mod!(pub diamond); // synthesized by LALRPOP
use crate::diamond::SingleParser;
use crate::diamond::PairParser;
use crate::diamond::UnwrappedPairParser;
use crate::diamond::FormattedPairParser;

#[test]
fn use_diamond() {
    assert!(SingleParser::new().parse("a").is_ok());
    assert!(PairParser::new().parse("a b").is_ok());
    assert!(UnwrappedPairParser::new().parse("a b").is_ok());
    assert!(FormattedPairParser::new().parse("a b").is_ok());
}

fn main() {
    let parser = SingleParser::new(); // Creating a closure to avoid repeating the parser creation for each parse call
    let parse = |input| parser.parse(input);

    println!("SingleParser: Input a => {}", parse("a").unwrap());

    let parser = PairParser::new();
    let parse = |input| parser.parse(input);
    println!("PairParser: Input a b => {:?}", parse("a b").unwrap());

    let parser = UnwrappedPairParser::new();
    let parse = |input| parser.parse(input);
    println!("UnwrappedPairParser: Input a b => {:?}", parse("a b").unwrap());

    let parser = FormattedPairParser::new();
    let parse = |input| parser.parse(input);
    println!("FormattedPairParser: Input a b => {:?}", parse("a b").unwrap());
}