# Make a language in Rust - Eldiro

## About

Language parser and interpreter for a fictional language called eldiro.
Original [tutorial](https://lunacookies.github.io/lang) from [lunacookies](https://github.io/lunacookies)

I'm planning to expand on this project and add support for types, functions
and control flow, as well as importing components from other local files.

Implemented:

- [x] Lexer
- [x] Parser
- [x] Interpreter
- [x] Integers and operators
- [x] Variables (def and use)
- [ ] Type checker
- [ ] Functions
- [ ] Control flow
- [ ] Imports

## Usage

```bash
$ cargo run --release -- examples/hello.eldiro
Hello, world!
```
