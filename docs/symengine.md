# Symengine
## Overview
The purpose of the `symengine` module is to provide an engine that will accept various strings, interpret them and execute the given commands.  The following code should cause the engine to do the following:
* Determine validity of input
* Parse the string and use this to construct an expression
    * The first command maps the string key "x" to the value 5
    * THe second command maps the result of the calculation to the "ans"
        * The second expression will store nothing and return a failure message if processed without anything assigned to the variable "x".
    * The third command stores a function of the variables "x" and "y" to the key "f". Does not require the previous existence of "x" and "y".

```rust
let mut engine = Engine::new();
let command = String::from("x = 5");
let command = String::from("(1.25 + x) * exp(x^2))");
let function = String::from("func f(x, y) = 2x + y^2");
engine.process_string(&command);
```