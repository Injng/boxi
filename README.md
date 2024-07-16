# boxi
Binary, Octal, and heXa(decimal) Interpreter

## Explanation
This was a program made out of a need to quickly do simple arithmetic between binary, octal, hexadecimal, and decimal numbers.
There is no complicated notation, all that is required is standard mathematical notation and 0b, 0o, or 0x for binary, octal,
and hexadecimal numbers respectively (decimal numbers do not need a prefix).

The program first parses the expression given into a series of tokens, with each token being either a number or operator. This
series of tokens is then converted into a postfix notation (à la RPN) using a shunting yard algorithm. This postfix notation is
then evaluated using a stack-based algorithm, which then returns the result. The result, by default, is printed in all four
mathematical bases.

