// https://lalrpop.github.io/lalrpop/quick_start_guide.htmls
fn main() {
    // process_root() will read the grammar file and generate the parser code.
    //  The default settings is to take files in src/ ending with the .lalrpop extension, and 
    // generates corresponding Rust source files with the same name in OUT_DIR.
    // OUT_DIR isan environment variable provided by Cargo to build scripts. 
    // Specifically, it points to target/debug/build/calculator-<hash>/out/
    // loo for a file named calculator1.rs in the OUT_DIR, which is generated from calculator1.lalrpop in src/.
    // process_root() returns a Result<(), lalrpop::Error>, so we call unwrap() to panic on any errors.
    lalrpop::process_root().unwrap();
}