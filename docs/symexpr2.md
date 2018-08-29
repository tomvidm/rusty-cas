# Rusty CAS
## Implementation
### Data Structures for Symbolic Expressions
Symbolic expressions are represented by a variant type `Expr`, which implements methods shared by all the underlying types.
#### Numeric
Numeric types are represented by a variant type that is either float, complex or integer valued.
#### Variables
Variables are represented by a string key, and will only carry meaning when provided with a string-expr mapping. This way, variables only work with a valid hash map, which should be constructed alongside any manipulation of an expression. A variable may indirectly refer to any expression.

Some mechanism must be implemented to prevent recursive expressions, where some variable A points to an expression F which also depends on A. 

### Simplification of expressions
Simplification of expressions should be provided as an option, and not enforced. A few simple rules should be provided to allow some "cleanup" of expressions. The following rules should be applied for cleanup:

```
0 * a = 0
1 * a = a
0 + a = a
a / a = 1
```

### Reduction to Canonical Form
If possible, expression such as `1 - x` and `-x + 1` should be considered equivalent. One step towards this is to keep expressions in some lexicogrpahic order when reducing to canonical form. Another idea is to let `-x` be represented by a unary operator on the argument `x`, so at `1 - x` is instead represented as `1 + (-x)`. This might be useful, as addition is commutative. (What about an analogue for multiplication and division?)