# Rust Out-of-Bounds Vector Access

This repository demonstrates a common error in Rust: accessing an element of a vector using an index that is out of bounds.  This will cause a runtime panic.

The `bug.rs` file contains the erroneous code, while `bugSolution.rs` shows how to fix the problem using safe Rust techniques.

## How to reproduce

1. Clone this repository.
2. Navigate to the root directory.
3. Run `rustc bug.rs && ./bug` to see the panic.
4. Run `rustc bugSolution.rs && ./bugSolution` to see the corrected output.
