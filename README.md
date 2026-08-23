See 

1. https://lalrpop.github.io/lalrpop
2. https://github.com/lalrpop/lalrpop/tree/master/doc/calculator

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

```
➜  lalrpop-calculator git:(calculator1) ✗ cargo run --bin main    
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
     Running `target/debug/main`
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

## Running location.larlpop

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

## Running buildingast.lalrpop

See 
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