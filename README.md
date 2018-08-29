# Rusty CAS
## Overview of features
A simple computer algebra system written in Rust.

Currently, it correctly handles simple expressions containing the following:
* Constant floats and integers
* Variables are keys to a hash map storing key-expression pairs
* Unary functions, such as `sin, cos, exp, sqrt & negation`
* Binary function, such as `add, sub, mul, div, pow`

Rusty CAS also supports taking the derivatives of expressions based on any variable, using the `get_derivative` method.

During construction of expressions, the module uses simple rules to clean up any expression `E` of the forms:
``` python
0 * a = 0
1 * a = a
0 + a = a
a / a = 1
```

## Planned features
* Extend the Expression types to include polynomials, sums and products
* Support symbolic integration of expressions
* Solving of equalities and inequalities
* Expansion of expressions as a reversible process `a(b + c) = ab + ac`