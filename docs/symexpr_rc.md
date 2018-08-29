## Implementation notes for Expr with reference counting

```rust
Rc::clone(&some_rc) // Avoids deep copy
let a = Rc::new(Expr::from_float(1.));  // Allocate on heap
let b = Rc::new(Expr::from_float(2.));  // Allocate on heap
let c = Expr::add(a, b);
```