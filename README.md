# rustbf
A brainfuck interpreter (maybe compiler?) in rust


# Usage:
1. Find out how Brainfuck works [here](https://en.wikipedia.org/wiki/Brainfuck)

`cargo run {filename}`

- The file has to have an extension of .bf
- Any characters or words other than > < + - , . [] = are regarded as comments by the interpreter
- Use `=` to output the literal value of the cell, and use `.` to output the ascii character corresponding to the cell's value.

