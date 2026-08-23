
LALRPOP is a Rust parser generator framework. To learn about LALRPOP,
see 

1. https://lalrpop.github.io/lalrpop
2. https://github.com/lalrpop/lalrpop/tree/master/doc/calculator

This repo contains my experiments reading the LALRPOP documentation and trying to understand how it works. The code is based on the examples in the LALRPOP documentation, but I have made some modifications.

To learn Rust use 

- https://rust-lang.org/learn/
- https://rust-book.cs.brown.edu/
- https://github.com/crguezl/the-rust-programming-language-exercises
- https://rustlings.rust-lang.org/

## First Steps

See 

- Section [Quick Start Guide](https://lalrpop.github.io/lalrpop/quick_start_guide.html) for the initial steps.
- [Cargo.toml](Cargo.toml) for the dependencies used in this project.
- [build.rs](build.rs) for the build script that runs LALRPOP on the .lalrpop files.


## lalrpop help

```
➜  lalrpop-calculator git:(calculator1) ✗ lalrpop --help
Usage: lalrpop [options] <inputs>...
       lalrpop --help
       lalrpop (-V | --version)

Options:
    -h, --help           Print help.
    -V, --version        Print version.
    -l, --level LEVEL    Set the debug level. (Default: info)
                         Valid values: quiet, info, verbose, debug.
    -o, --out-dir DIR    Sets the directory in which to output the .rs file(s).
    --features FEATURES  Comma separated list of features for conditional compilation.
    -f, --force          Force execution, even if the .lalrpop file is older than the .rs file.
    -c, --color          Force colorful output, even if this is not a TTY.
    --no-whitespace      Removes redundant whitespace from the generated file. (Default: false)
    --comments           Enable comments in the generated code.
    --report             Generate report files.
```

## Running calculator1.lalrpop

See

- [src/calculator1.lalrpop](src/calculator1.lalrpop)
- [src/use_calculator1.rs](src/use_calculator1.rs)
  
```
➜  lalrpop-calculator git:(calculator1) ✗ cargo run --bin calculator1
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.61s
     Running `target/debug/calculator1`
The number 22 ends at position 2
22 = 22
The number 22.5 ends at position 5
(22.5) = 22.5
The number 22 ends at position 6
((((22)))) = 22
The number 22 ends at position 4
Error example: ((22) = Unrecognized EOF found at 5
Expected one of ")"
```

## Running location.lalrpop

Based on: https://lalrpop.github.io/lalrpop/tutorial/002_paren_numbers.html

Parses parenthesized numbers (`(((22)))`) and uses the `@L`/`@R` markers to
capture the byte offsets where a matched `Num` token starts and ends, printing
them alongside the parsed value.

See

- [src/location.lalrpop](src/location.lalrpop)
- [src/use_location.rs](src/use_location.rs)

```
➜  lalrpop-calculator git:(calculator1) cargo run --bin location            
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/location`
The number 22 starts at position 0 and ends at position 2
22 =at pos=> 22
The number 22.5 starts at position 1 and ends at position 5
(22.5) =at pos => 22.5
The number 22 starts at position 4 and ends at position 6
((((22)))) =at pos => 22
The number 22 starts at position 2 and ends at position 4
Error example: ((22) Unrecognized EOF found at 5
Expected one of ")"
The number 22 starts at position 0 and ends at position 2
Using NumParser: 33 => 22
```

## Running diamond.lalrpop

Based on: https://lalrpop.github.io/lalrpop/tutorial/003_type_inference.html#type-inference

The `<>` ("diamond") operator lets you avoid writing out an explicit action
when you just want to select (and possibly reassemble) the values matched by
a rule, instead of writing an explicit `<x:A> <y:B> => ...` action for every
production. [src/diamond.lalrpop](src/diamond.lalrpop) shows several
equivalent ways of writing the same rule with and without `<>`, including how
it interacts with tuple patterns (`UnwrappedPair`) and `format!` (`FormattedPair`).

See

- [src/diamond.lalrpop](src/diamond.lalrpop)
- [src/use_diamond.rs](src/use_diamond.rs)

```
➜  lalrpop-calculator git:(calculator1) ✗ cargo run --bin diamond
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.23s
     Running `target/debug/diamond`
SingleParser: Input a => a
PairParser: Input a b => ("a", "b")
UnwrappedPairParser: Input a b => "ab"
FormattedPairParser: Input a b => "a b"
```

## Running handling1.lalrpop

Based on: https://lalrpop.github.io/lalrpop/tutorial/004_full_expressions.html

A first cut at a full arithmetic expression grammar with `+`, `-`, `*`, `/`
and parentheses. Precedence and left-associativity are encoded "by hand" via
the classic layered-nonterminal trick (`Expr` → `Term` → `Factor` → `Number`),
the same technique used in [src/calculator1.lalrpop](src/calculator1.lalrpop),
rather than via LALRPOP's `#[precedence]`/`#[assoc]` annotations (compare
with [handling2.lalrpop](#running-handling2lalrpop) below).

See

- [src/handling1.lalrpop](src/handling1.lalrpop)
- [src/use_handling1.rs](src/use_handling1.rs)

```
➜  lalrpop-calculator git:(calculator1) ✗ cargo run --bin handling1
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/handling1`
3-2-1 = 0
2+3*5 = 17
(4-2)*2 = 4
Error example: ((22) = Unrecognized EOF found at 5
Expected one of ")", "+" or "-"
```

## Running handling2.lalrpop

Based on: https://lalrpop.github.io/lalrpop/tutorial/004_full_expressions.html

The same arithmetic expressions as `handling1.lalrpop`, but this time the
grammar has a single `Expr` nonterminal and precedence/associativity are
declared explicitly with the `#[precedence(level = "N")]` and
`#[assoc(side = "left")]` attributes instead of being encoded through
separate `Expr`/`Term`/`Factor` layers. Lower `level` values bind tighter
(`*`/`/` at level `1` bind tighter than `+`/`-` at level `2`).

See

- [src/handling2.lalrpop](src/handling2.lalrpop)
- [src/use_handling2.rs](src/use_handling2.rs)

```
➜  lalrpop-calculator git:(calculator1) ✗ cargo run --bin handling2
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/handling2`
3-2-1 = 0
2+3*5 = 17
(4-2)*2 = 4
Error example: ((22) = Unrecognized EOF found at 5
Expected one of ")", "+" or "-"
```

## Running buildingast.lalrpop

See 
- [Section Building ASTs](https://lalrpop.github.io/lalrpop/tutorial/005_building_asts.html#building-asts) in the LALRPOP tutorial.
- [docs/ast.md](docs/ast.md)
- [src/ast.rs](src/ast.rs)
- [src/use_buildingast.rs](src/use_buildingast.rs)
- [src/buildingast.lalrpop](src/buildingast.lalrpop)


```
➜  lalrpop-calculator git:(calculator1) cargo run --bin ast                                                                                         
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.05s
     Running `target/debug/ast`
3-2-1 = Sub(Sub(3, 2), 1)
2+3*5 = Add(2, Mul(3, 5))
(4-2)*2 = Mul(Sub(4, 2), 2)
Error example: ((22) = Unrecognized EOF found at 5
Expected one of ")", "+" or "-"
```