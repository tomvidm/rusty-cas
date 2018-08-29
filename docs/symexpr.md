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

### How to increase speed
Currently, an Expr can be allocated on the stack. However, the subexpressions are stored as smart pointers to some allocated Expr on the heap. This causes a lot of pointer jumping when traversing the expression tree, especially with the addition of variables that require traversing a hash map.

How can this be improved? Observations:
* The hash map implementation of variables is flexible and seemingly robust. However, no mechanism is in place for cleaning up the map. 
* All expressions are cloned and put into a new box when assigned as a subexpression. This causes a lot of unnecessary allocations on the heap, and a smart pointer with reference counting might be considered instead of `Box<Expr>`.
* Preallocate a vector containing `Expr` on the heap and store indexes to subexpressions instead. Consider creating a memory pool class for this.