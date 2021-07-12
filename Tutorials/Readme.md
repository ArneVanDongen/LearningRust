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

### ~~Rust Tuples~~
<details>
<summary>Notes</summary>

  Tuple is a group of data, elements inside it don't have names. Used to group things without constructing complex objects.
  
  Created in rust like so: `let some_tuple = (2, 3.4);` And accessed with dot notation `println!("My data is {} {}", some_tuple.0, some_tuple.1);`
  
  Can contain any collection of datatypes, and any amount of elements.
  
  Getting elements of nested tuptles can be done by using spaces after the first index `some_tuple.5 .2` or by using parentheses `(some_tuple.5).2`.
  
  You can populate multiple variables from a tuple: `let (red, green, blue) = get_some_rgb();`.
  
  The empty tuple, or unit tuple, `()` is like void, or empty closure.
  
  Be mindful of what your data represents, and create data types to properly describe your data.
</details>

### ~~Rust Structs, Traits and Impl~~
<details>
<summary>Notes</summary>

  Structs represent complex data types, they act like objects, but are different:
  
  * Inheritance can't be done.
  * Polymorphism can be done through Traits.
  * Structs can have methods.
  
  It's nice to sort fields in a struct alphabetically.
  
  If you want to be able to edit fields in a struct, declare the var that holds the struct instant as `mut` and all fields will become mutable.
  
  You can copy values from another instance of the same struct by passing it into the constructor:
  ```Rust
  let some_struct_2 = SomeStruct {
    field1: 22,
    ..some_struct_1
  }
  ```
  
  If using a Struct declared in a different file, use the `pub` keyword where its declared to make it visible outside that file. This applies to fields within the structs as well.
  
  If you want to do something like inheritance, use composition instead.
  ```Rust
  struct AnotherStruct {
    an_additional_int: i32,
    some_struct: SomeStruct,
  }
  ```
  
  Methods are defined outside the struct definition using the `impl` keyword. These are associated functions.
  ```Rust
  impl AnotherStruct {
    pub fn some_fuction(param: bool) -> i32 {
      if param { 1 } else { 2 };
    }
  }
  ```
  
  When implementing associated functions you can use `Self` to represent the struct you're implementing for.
  
  To use data from the struct instance itself in an associated function, set the first parameter to `&self`:
  ```Rust
  impl AnotherStruct {
    pub fn is_smaller(&self, compare_to: i32) -> bool {
      self.an_additional_int < compare_to
    }
  }
  ```
  
  The `&self` is assumed when a method like this is called and doesn't need to be entered manually: `another_struct.is_smaller(9)`.
  
  Calling method can be done through `::` or by `.`. If the `&self` keyword is used, use the `.` dot notation, else use the `::` notation.
  
  Traits are for polymorphism (treating different structs the same).
  ```Rust
  impl SomeTrait for AnotherStruct {
    fn is_valid(&self) -> bool {
      self.an_additional_int > 0
    }
  }
  ```
  
  Now AnotherStruct can be used alongside other structs that have `SomeTrait`.

  Traits can be used in method definitions like so:
  ```Rust
  fn print_if_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
      println!("We're valid");
    }
  }
  ```
  
</details>

### ~~Rust Enumerations~~
<details>
<summary>Notes</summary>

  Enums can have any struct as data:
  ```Rust
  enum Payment {
    Cash (f32),
    CreditCard,
    DebitCard,
  }
  let some_payment = Payment::Cash(100.);
  ```
  
  These can be strongly typed with explicit names as well:
  ```Rust
  enum Payment {
    Cash (f32),
    CreditCard,
    DebitCard,
    Crypto{accound_id: String, amount: f32},
  }
  ```
</details>

### ~~Rust Generics~~
<details>
<summary>Notes</summary>

  Structs must have unique names, so if you want to create multiple kinds of similar structs, you can create a generic struct.
  ```Rust
  struct Point<T> { // type param is specified as generic by angle brackets and upper camel case: <Aaa, Bbb, ...>
    x: T,
    y: T,
  }
  ```

  Enums and functions can be generics too.
  
  The compiler makes explicit versions of generics for every type it is used with.
        
  To make generics less generic, you can use constraints to specify what a type passed to the generic should be able to do. Constraints are specified with a colon after the Type definition. If you want to add multiple constraits, use the `+` operator after the first one.

  ```Rust
  fn add<T: std::ops::add<Output=T>>(a: T, b: T) -> T {
    a + b
  }
  ```

  If you have a lot of constraints on your generic you can use a where clause to improve readability of the method signature.

  ```Rust
  fn add<T>(a: T, b: T) -> T 
  where T: std::ops::add<Output=T> {
    a + b
  }
  ```

  Implementing for a genecic struct looks like so
  ```Rust
  struct Point<T> {
    x: T,
    y: T,
  }

  impl<T> Point
  where T: std::fmt::Debug { // Constraints go on the impl block
    fn log_something(&self) {
      println!("{:?} {:?}", self.x, self.y);
    }
  }
  ```
        
</details>

### Rust Ownership and Borrowing
<details>
<summary>Notes</summary>

  I had notes here before but forgot to commit them before restarting my pc -_-
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
