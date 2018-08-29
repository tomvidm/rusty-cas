# Rusty CAS
## Implementation
### Data Structures for Symbolic Expressions
Symbolic expressions are represented by a variant type `Expr`, which implements methods shared by all the underlying types.
#### Numeric
Numeric types are represented by a variant type that is either float, complex or integer valued.
#### Variables
Variables are represented by a string key, and will only carry meaning when provided with a string-expr mapping. This way, variables only work with a valid hash map, which should be constructed alongside any manipulation of an expression. A variable may indirectly refer to any expression.