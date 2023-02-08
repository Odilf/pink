# pink

A very minimal replecement based language. Intended for ressembling as closely as possible the "math way" to write stuff, in an extremely declarative fashion. 

## Documentation

Each file corresponds to a structure. At the top of each file you have to delcare it's domain and a set of reserved keywords.

```pink
domain { true, false }
reserve { in } # Commas, curly braces and parenthesis are reserved by the runtime itself
```

After that, you can have a series of definitions. A definition may have concrete elements from the domain or literals from the reserved keywords. Anything else is considered a variable (really, *anything*, including for example square brackets).

```pink
x in { x } = true;
```

If a variable ends with `...`, then it can capture an arbitrary amount of items, not only 1 (including 0)

```pink
x in { x, rest... } = true;
x in { y, rest... } = x in { rest... };
```

In the REPL you can evaluate expressions. An expression only has elements and literals (no variables).

```
>> false in { false }
true
```