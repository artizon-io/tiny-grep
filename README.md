# Rust Grep

A `grep` terminal utility written in Rust

## Knowledge

Resources:
- [Rust book - chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Rust book - chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust book - chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust book - chapter 5](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust book - chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust book - chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust book - chapter 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust book - chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust book - chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust book - chapter 11](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust-by-example - chapter 19 - Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)

### Rust Variables & Types

- Shadowing: the act of re-declaring a variable that is already in-scope. Re-declaring the same variable in the same scope is allowed
- Scalar types:
  - Integers: `u8` (unsigned), `i8` (signed), `u16`, `i16`, ..., `usize`, `isize` (size depends on arch)
  - Floats: `f32` and `f64`
  - `bool` (1 byte)
  - `char` (4 bytes; any valid Unicode)
- Compound types:
  - Tuple (e.g. `(i32, f64)`). Indexed using e.g. `.0`. A *unit* represents an empty tuple `()`, which is used to denote empty value
  - Array (e.g. `[i32; 5] `). Stored on *stack* (fixed size). Can be initialised using e.g. `[3; 5]` syntax
- Integer literals:
  - Decimal: `100_000`
  - Hex: `0xff`
  - Octal: `0o00`
  - Binary: `0b0100_0100`
  - Byte (`u8` only): `b'A'`
- Integer overflows are checked in `debug` mode, but not in `release`
- Explicitly check for overflows using built-in methods inside `std`:
  - `wrapping_*()`
  - `checked_*()`
  - `overflowing_*()`
  - `saturating_*()`

### Rust Expressions vs Statements & Control Flows

- Expressions doesn't end in semicolon `;`
- Assignment is *not* an expression
- Block `{}` is an expression
- Control flows - `if`, `match`, `loop`, ... are expressions
- Loops: `loop`, `while`, `for...in` (iterate over collection)
- `break`/`continue` superpowers: `break 'loop_label;` and `break return_val;`

### Rust Range & Slice

- Range: e.g. `1..5`
- Slice: reference a contiguous sequence of elements in a collection 

### Rust Ownership & Borrowing

- Ownership rules:
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
- By default value is `move` (heap-data; implement `Drop` trait) or `copy` (stack-data; implement `Copy trait`). Value can be deep-cloned using `clone()`
  - Anything that allocates data (on heap), or is some form of resource cannot implement `Copy`
- Reference `&`; De-reference `*`
- Reference scope: starts from `&`, and continues through the last time that reference is used, or dropped
- References rules:
  - Can have any number of immutable references (in the same reference scope)
  - Can only have at most 1 mutable reference (in the same reference scope)
- Automatic referencing/de-referencing: automatically insert `*` or `&` in order to match the signature of the function

### Rust `String` and string literals

- `String`: a smart pointer, with interior data stored on heap. Can be mutated
- String literal `str`: a string slice references read-only memory (size known at compile time). Cannot be mutated
- Convert `str` to `String` using `String::from()`
- Functions that take `&str` (string slice) as argument can also take `&String` as argument. `&String` is implicitly deref coerced into `&str` (`Deref` trait)

### Rust `struct`

- When the struct instance is assigned to a reference of another struct, stack-value fields are `copy`, heap-value fields are referenced.
- When the struct instance is assigned to an instance of another struct, stack-value fields are `copy`, heap-value fields are `move`.
- Tuple struct: struct without named fields. Indexed like regular tuple
- Unit struct: struct without fields
- Method's first argument is the struct instance itself
- When inside `impl`: `&self` is shorthand for `self: &Self`, and `Self` is shorthand for the struct e.g. `Rectangle`
- Can take ownership of the struct instance (`self`), or borrow immutably (`&self`), or borrow mutably (`&mut self`)
- Define associated functions (static methods) e.g. `new` the same way as methods but without `self` as first argument

### Rust print

- `println!()` macro: takes reference
- `dbg!()` macro: takes ownership and then return ownership. Print to `stderr`
- No format specifiers: `Display` (`Display` trait needs to be implemented)
- Format specifier `:?`(or `:#?`): `Debug` (`Debug` trait needs to be implemented). `Debug` trait can be automatically implemented using `![derive(Debug)]` attribute

### Rust Enum

- E.g. `Option`, `Result`
- Useful combination with `match`, and `if let` expressions. Both enable smart pattern matching with enum
- Variants of the enum are namespaced under its identifier
- Optionally associate values, *unnamed* or *named*, to an Enum variant e.g. `IpAddr::V4(127, 0, 0, 1)` (initialisation), `Position { x: i32, y:i32 }` (type definition)
- Can also `impl` methods on enum. Methods will be defined on the enum value e.g. `Fruit::Apple.wash()`. The enum value will be the `Self` inside `impl`

### Rust Module System

- Binary crate: must contains `main` function, either inside `main.rs` or `bin/`
- Library crate: either inside `lib.rs`, or `lib/`
- Package: bundle of 1 or more crates. Define by `Cargo.toml`
- `use`: brings a path into scope. Can nest/glob imports using `{}`, `*`, and `self`
- By default module would not be included. Must explicitly tell Rust compiler to look for it by `mod moduleName;`
- Module can be defined inside `moduleName.rs` (relative), `moduleName/mod.rs` (relative; older-style; not recommended), or inline `moduleName {}`
- Can use the crate name e.g. `crateName::moduleName` and `super` ato address modules
- Re-export imported path using `pub use`

### Rust Tests

