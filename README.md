# Multiple Mutable Borrows in Rust
This example demonstrates a common error in Rust: creating multiple mutable borrows of the same variable.  Rust's ownership system prevents this to avoid data races and ensure memory safety. 

The `bug.rs` file contains code that attempts to create two mutable references (`y` and `z`) to the same variable `x`. This will result in a compile-time error.