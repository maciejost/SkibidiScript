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
then transformed to JavaScript using OXC.

To run the program, use the following command:
```bash
cargo run <path-to-directory>
```

## Examples
A full list of SkibidiScript keywords and their TS counterparts can be found in  [EXAMPLES.md](EXAMPLES.md).

A function written in SkibidiScript and its corresponding output: 
```ts
beGone aura Title = "Produkt" | "Usługa" | "Kontakt" | "O nas" | "Kariera";


skibidi pickTitle(value) {
    riddleMeThis (!Title.includes(value)) {
        throwItBack crisp Error('expected a valid title');
    }

    letHim (value) {
        cook "Produkt":
            hesDoneFor "Nasze produkty";
        cook "Usługa":
            hesDoneFor "Nasze usługi";
        cook "Kontakt":
            hesDoneFor "Kontakt z nami";
        cook "O nas":
            hesDoneFor "O nas";
        cook "Kariera":
            hesDoneFor "Pracuj z nami";
    }

    hesDoneFor value;
}

beGone slay pickTitle;

```

```js
function pickTitle(value) {
	if (!Title.includes(value)) {
		throw new Error("expected a valid title");
	}
	switch (value) {
		case "Produkt": return "Nasze produkty";
		case "Usługa": return "Nasze usługi";
		case "Kontakt": return "Kontakt z nami";
		case "O nas": return "O nas";
		case "Kariera": return "Pracuj z nami";
	}
	return value;
}
export default pickTitle;
```


## Local development
Make sure to have [Rust installed](https://www.rust-lang.org/tools/install) on your machine.
