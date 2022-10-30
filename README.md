# rust interpreter
idk what to actually call it  
  
interpreter for low level whitespace-separated operations  
this is literally my first rust project its pretty bad
  
# running examples
the examples have .asm extension for syntax highlighting, they arent actual assembly  
  
```
cargo run examples/hello.asm
```
  
or download from [releases](https://github.com/raxracks/interpreter-rs/releases) and run
```
interpreter-rs <filename>
```

# Normal Operations
| Op Name | Param 1 | Param 2 | Param 3 | Stack Top After | Description
|-|-|-|-|-|-|
| sym | symbolName | symbolValue | | | Defines symbol and gives it a value
| push | value | | | | Pushes value to top of stack
| pop | | | | | Pops top of stack to nowhere
| pops | symbolName | | | | Pops top of stack onto symbol
| label | labelName | | | | Define a label you can jump to
| jmp | labelName | | | | Jump to label
| je | labelName | | | | Jump to label if top 2 values in stack are equal
| jne | labelName | | | | Jump to label if top 2 values in stack are not equal
| jlt | labelName | | | | Jump to label if top of stack is less than value under
| jgt | labelName | | | | Jump to label if top of stack is greater than value under
| jle | labelName | | | | Jump to label if top of stack is less or equal to the value under
| jge | labelName | | | | Jump to label if top of stack is greater or equal to the value under
| dumps | symbolName | | | | Dump stack onto symbol
| res | symbolName | | | | Restore stack from symbol
| print | value | | | | Prints value to stdout
| exit | exitCode | | | | Exits program with exit code
| clears | | | | | Clear stack
| ret | | | | | Return to position where jmp occured

# Math Operations
| Op Name | Param 1 | Param 2 | Param 3 | Stack Top After | Description
|-|-|-|-|-|-|
| add | | | | Addition of 2 values | Pops 2 items off the stack and adds them
| sub | | | | Subtraction of 2 values | Pops 2 items off the stack and subtracts them
| mul | | | | Multiplication of 2 values | Pops 2 items off the stack and multiplies them
| div | | | | Division of 2 values | Pops 2 items off the stack and divides them
| mod | | | | Modulus of 2 values | Pops 2 items off the stack and modulos them
| pow | | | | Power of 2 values | Pops 2 items off the stack and exponents (great english) them
| inc | | | | Incremented value | Pops item off stack and increments it
| dec | | | | Decremented value | Pops item off stack and decrements it