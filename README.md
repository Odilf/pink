# pink

A very minimal replacement based DSL, intended for math experimentation. 

## Get started

To install pink as a binary, if you have cargo just do 

```bash
cargo install pink-runtime
```

Then, to run a program you can run 

```bash
pink-runtime [PATH]
```

To see more information you can do 

```bash
pink-runtime --help
```

To use pink as a library, you can add it as any other crates.io dependency (though I would recommend to use it as a git dependency). 

## Documentation

### Head

Each file corresponds to a structure. At the top of each file you have to delcare it's domain, a set of reserved keywords and it's dependencies.

```pink
domain { true, false }
reserve { in } # Commas, curly braces and parenthesis are reserved by the runtime itself
use { }
```

### Definitions

After the head, you can have a series of definitions. A definition may have concrete elements from the domain or literals from the reserved keywords. Anything else is considered a variable (really, *anything*, including mathematical symbols).

```pink
true and true => true;
p and q => false;
```

Expressions are matched from top to bottom. So, while `p` and `q` are normally able to bind to `true`, given the order of the definitions here we won't ever reach that case. 

#### Spread variables

If a variable ends with `...`, then it can capture an arbitrary amount of items (but at least 1 (this might change)).

```pink
x in { } => false;

x in { x } => true;
x in { y } => false;

x in { x, rest... } => true;
x in { y, rest... } => x in { rest... };
```

### Matching

The runtime matches every possible subexpression and finds the result with the least number of tokens in the end. 

### REPL

In the REPL you can evaluate expressions. For now, an expression only has elements and literals (no variables).

```
>> false in { false }
true
```
