# SkibidiScript

A brainrotted TypeScript flavour written in Rust.

## Usage
SkibiScript is a flavour of TypeScript, so all your prior knowledge of TypeScript is applicable here. [EXAMPLES.md](EXAMPLES.md)
contains a list of all keywords and their TypeScript counterparts, any keyword not mentioned in the file is treated as
a regular TypeScript keyword.

All SkibidiScript files must have a `.skibidi` extension. To use jsx in SkibidiScript, the file must have a `"use skibidi tsx";`
directive on the first line, akin to React 19´s `"use client"` and `"use server"` directives.

### Transpiling
The program takes a path to a directory containing `.skibidi` files and transpiles them down to TypeScript. The transpiled files are
then transpiled once more to JavaScript using the swc. The transpiled files are placed in a `dist` directory in the input directory.

To run the program, use the following command:
```bash
cargo run <path-to-directory>
```

## Examples
A full list of SkibidiScript keywords and their TS counterparts can be found in  [EXAMPLES.md](EXAMPLES.md).


## Local development
Make sure to have [Rust installed](https://www.rust-lang.org/tools/install) on your machine.
