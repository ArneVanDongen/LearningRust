# Rust Tutorials

## Rust Tutorial by Doug Milford

~~Rust Primitives (basic data)~~
<details>
<summary>Notes</summary>

  Nothing too interesting, basic variable declaration stuff.
  `snake_case` convention in rust for naming things.
  Compiler figures out what type a variable will be, so no need to explicitely define it (but you can with `: type`)
  Everything is immutable by default, yay.
  `i8, i16` types for integers, `ux` for zero and positive only.
  Overflowing a variable cause a panic while running in debug mode (`cargo run`) but will wrap around (`let mut x: i8 = 120; x += 10; println!(x)` results in -126) in release mode (`cargo run --release`).
  `isize` and `usize` creates a variable with a size based on the computer's architecture (32 vs 64 bit these days).
  Floats only have 32 and 64 bit variants. Don't forget the dot.
  Chars are more than ascii.
</details>
Rust String vs str slices
<details>
<summary>Notes</summary>

stuff
</details>
Rust Functions and Procedures
<details>
<summary>Notes</summary>

stuff
</details>
Rust Conditional Statements
<details>
<summary>Notes</summary>

stuff
</details>
Rust Tuples
<details>
<summary>Notes</summary>

stuff
</details>
Rust Structs, Traits and Impl
<details>
<summary>Notes</summary>

stuff
</details>
Rust Enumerations
<details>
<summary>Notes</summary>

stuff
</details>
Rust Generics
<details>
<summary>Notes</summary>

stuff
</details>
Rust Ownership and Borrowing
<details>
<summary>Notes</summary>

stuff
</details>
Rust Lifetimes
<details>
<summary>Notes</summary>

stuff
</details>
Rust Casting, Shadowing, Consts and Static
<details>
<summary>Notes</summary>

stuff
</details>
Rust println! and format! macros
<details>
<summary>Notes</summary>

stuff
</details>
Rust lib vs main
<details>
<summary>Notes</summary>

stuff
</details>
Rust Cargo.toml
<details>
<summary>Notes</summary>

stuff
</details>
Rust Cargo and Rustup Commands
<details>
<summary>Notes</summary>

stuff
</details>
Rust 3D Graphics in the Browser: Boilerplate Setup and WASM Introduction
<details>
<summary>Notes</summary>

stuff
</details>
Rust 3D Graphics in the Browser: 2D Graphics
<details>
<summary>Notes</summary>

stuff
</details>
Rust 3D Graphics in the Browser: 3D Graphics
<details>
<summary>Notes</summary>

stuff
</details>
