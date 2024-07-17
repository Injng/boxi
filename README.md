# boxi
Binary, Octal, and heXa(decimal) Interpreter

## Installation
### Linux
A binary release is available for Linux x86_64 machines. To install, download the binary release from the releases page
and execute it. You may need to make the binary executable or place it somewhere on your PATH.

### Development
For development purposes, clone the repository source code locally. As this project is written in Rust, a working `cargo`
installation is required. To build the project, run the following command:
```bash
cargo build
```
To run the project, run the following command:
```bash
cargo run
```

## Usage
The program may be used as either a one-off command or as a REPL. The REPL can be invoked by running `boxi` with no arguments.
Otherwise, the program expects the first argument to be a mathematical expression. You may also pass the `--verbose` flag 
*after* the expression to see the intermediate steps of the evaluation. Basic usage examples can be printed in the terminal with
`boxi --help`.

Boxi supports integer arithmetic in binary, octal, hexadecimal, and decimal bases. Binary numbers must be prefixed with `0b`,
octal numbers with `0o`, and hexadecimal numbers with `0x`. Decimal numbers do not need a prefix. The program will output the
result in all four bases. This is particularly handy for converting single numbers, as the program will return the same number
in all four bases.

Boxi currently supports four operators: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`). The program
expects each operator to operate on two numbers, in infix notation (i.e. 1 + 2). As the program removes all whitespace, any
whitespace between numbers and operators is ignored. Boxi follows the order of operations, with multiplication and division
taking precedence over addition and subtraction. All four operators are left-associative.

You may change the order of operations with parantheses. Parantheses are evaluated first, with the innermost parantheses being
evaluated first. Parantheses may be nested to any depth. Note that implicit multiplication is not supported, so all multiplication
operations must be explicitly stated.

WARNING: as boxi currently does not support floating point numbers, division will always return an integer.

### Examples
```
0xdeadbeef
1 + 2
0x123 * 0x145
0o134 - 0b10101
0x4 / 0b10
(1 + 24) * 3
```

## Explanation
This was a program made out of a need to quickly do simple arithmetic between binary, octal, hexadecimal, and decimal numbers.
There is no complicated notation, all that is required is standard mathematical notation and 0b, 0o, or 0x for binary, octal,
and hexadecimal numbers respectively (decimal numbers do not need a prefix).

The program first parses the expression given into a series of tokens, with each token being either a number or operator. This
series of tokens is then converted into a postfix notation (à la RPN) using a shunting yard algorithm. This postfix notation is
then evaluated using a stack-based algorithm, which then returns the result. The result, by default, is printed in all four
mathematical bases.

## Roadmap
- [ ] Add support for more operators
- [ ] Add support for floating point numbers
- [ ] Add support for functions
- [ ] Add support for variables

