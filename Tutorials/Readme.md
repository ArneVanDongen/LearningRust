# Rust Tutorials

## Rust Tutorial by Doug Milford

### ~~Rust Primitives (basic data)~~
<details>
<summary>Notes</summary>

  Nothing too interesting, basic variable declaration stuff.
  
  `snake_case` convention in rust for naming things.
  
  Compiler figures out what type a variable will be, so no need to explicitely define it (but you can with `: type`).
  
  Everything is immutable by default, yay.
  
  `i8, i16` types for integers, `ux` for zero and positive only.
  
  Overflowing a variable cause a panic while running in debug mode (`cargo run`) but will wrap around (`let mut x: i8 = 120; x += 10; println!(x)` results in -126) in release mode (`cargo run --release`).
  
  `isize` and `usize` creates a variable with a size based on the computer's architecture (32 vs 64 bit these days).
  
  Floats only have 32 and 64 bit variants. Don't forget the dot.
  
  Chars are more than ascii.
</details>

### ~~Rust String vs str slices~~
<details>
<summary>Notes</summary>

  Rust strings are harder because complexity isn't hidden by the language
  
  ```
             | String | &str
  pronounced | String | string slice
  stored as  | u8s    | u8s
  stored on  | Heap   | Usually on the stack, sometimes ref to heap data, or embedded in code
  mut?       | Yes    | No (exceptions)
  ```
  One can easily translate between the two types.
  
  Strings is for mutating and holding data longer than the stack can, string slice is for runtime speed.
  
  From string slice to String can be done using `to_string()` or passing a it to `String::from()`.
  
  From String to string slice take the `&` reference of the string variable.
  
  To combine two string slices, put them in an array and call `.concat()` on it, or use the `format!()` macro.
  
  You can add a string slice to a String by using the `+` operator. The String NEEDS to go first.
  
  You can add a string slice to a String by making the String `mut` and using `push_str()`. Adding chars can be done by using `push()`.
  
  You can add two Strings by using `+` and referencing the second String (so it becomes a string slice).
  
  Taking a substring of a string slice can be done using brackets: `let s = "string slice"; let substring = &s[0..3]` which takes up to but not including. You can ommit either the first or last number. Overflowing will cause the program to panic.
  
  Getting the char at an index can be done like so: `&s.chars().nth(i)`, this is safer because it returns an Option.
</details>

### ~~Rust Functions and Procedures~~
<details>
<summary>Notes</summary>

  Functions and procedures are similar in the they both accept parameters and can call other code.
  
  Difference is in that functions return a value, while procedures do not.
  
  Omit the semicolon if you want to return a value, you can use the `return` keyword as well.
</details>

### ~~Rust Conditional Statements~~
<details>
<summary>Notes</summary>

  `false`, `true` and `==`. `()` not needed.
  
  Inline if statement can be done: `let var = if some_int == 9 { 300 } else { 400 };` Can contain else if as well.
  
  `match` statement has more capabilities than if. Example: `let var = match some_int { 9 => 200, 10..=100 => 300, _ => 400, };`. Match statements have to be exhaustive.
</details>

### Rust Tuples
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Structs, Traits and Impl
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Enumerations
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Generics
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Ownership and Borrowing
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Lifetimes
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Casting, Shadowing, Consts and Static
<details>
<summary>Notes</summary>

stuff
</details>

### Rust println! and format! macros
<details>
<summary>Notes</summary>

stuff
</details>

### Rust lib vs main
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Cargo.toml
<details>
<summary>Notes</summary>

stuff
</details>

### Rust Cargo and Rustup Commands
<details>
<summary>Notes</summary>

stuff
</details>

### Rust 3D Graphics in the Browser: Boilerplate Setup and WASM Introduction
<details>
<summary>Notes</summary>

stuff
</details>

### Rust 3D Graphics in the Browser: 2D Graphics
<details>
<summary>Notes</summary>

stuff
</details>

### Rust 3D Graphics in the Browser: 3D Graphics
<details>
<summary>Notes</summary>

stuff
</details>
