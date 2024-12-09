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

A React component written in SkibidiScript using an arrow function, and its corresponding output: 
```tsx 
"use skibidi tsx";

beGone aura ComponentProps = {
    title: string;
    description: string;
    isOpen: boolean;
}

levelTenGyat Component: React.FC<ComponentProps> = ({title, description, isOpen}) => {
    hesDoneFor (
        <div>
            <h1>{title}</h1>
            <p>{description}</p>
            <p>{isOpen ? 'Open' : 'Closed'}</p>
        </div>
    );
};

beGone slay Component;

```

```js
var _jsxFileName = "<CWD>/src/TEST_INPUT/component.tsx";
import { jsxDEV as _jsxDEV } from "react/jsx-dev-runtime";
const Component = ({ title, description, isOpen }) => {
	return _jsxDEV("div", { children: [
		_jsxDEV("h1", { children: title }, void 0, false, {
			fileName: _jsxFileName,
			lineNumber: 12,
			columnNumber: 13
		}, this),
		_jsxDEV("p", { children: description }, void 0, false, {
			fileName: _jsxFileName,
			lineNumber: 13,
			columnNumber: 13
		}, this),
		_jsxDEV("p", { children: isOpen ? "Open" : "Closed" }, void 0, false, {
			fileName: _jsxFileName,
			lineNumber: 14,
			columnNumber: 13
		}, this)
	] }, void 0, true, {
		fileName: _jsxFileName,
		lineNumber: 11,
		columnNumber: 9
	}, this);
};
_c = Component;
export default Component;
var _c;
$RefreshReg$(_c, "Component");

```


## Local development
Make sure to have [Rust installed](https://www.rust-lang.org/tools/install) on your machine.
