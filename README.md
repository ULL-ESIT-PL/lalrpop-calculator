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