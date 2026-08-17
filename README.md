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
➜  lalrpop-calculator git:(calculator1) ✗ cargo run --bin location
   Compiling calculator v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/LALRPOP/lalrpop-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.34s
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
```