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

### Rust Collections

- Interior data stored on *heap*
- Collection itself e.g. `String` is a *smart pointer*

`Vec<T>`
- Quick initialisation using `vec![1, 2, 3]` macro
- Access by index
  - `vec[]` might panic
  -  `vec.get()` returns `Option<&T>`. Never panics
- Operations on vector (e.g. `push`, `remove`) are a immutable/mutable borrow on the vector, which means immutably/mutably borrowing every element inside the vector

`String`
- Underneath is a `Vec<u8>`
- Signature of `+` (`add`): `fn add(self, s: &str) -> String`. Takes ownership of `Self`, and borrow immutably from the second `String` (coerced into `&str`)
- `format!` macro returns `String`
- Cannot directly index using `[]` because each character inside the string takes variable no. bytes to store
- Must iterate over characters/bytes using `.chars()` or `.bytes()`

`HashMap<K, V>`
- `.get(key)` returns `Option<&V>`
- Iterate inside `for...in` similar to `Vec<T>`. `HashMap` also has iterators
- Use `.entry(key).or_insert(value)` syntax to map a value to a key if key doesn't yet exist

### Rust Error Handling & `?` Operator

Recoverable Error
- In the form of `Result<T, E>`
- Unwrap using `unwrap()`, `expect()`, `match`, `if let`, `?`
- `<expression>?`: shortcut that return the error (in the function) if `<expression>` evals to an error. Else, evals to the `OK` value
- Underneath `?` will call the `from` function (`From` trait, e.g. `impl From<io::Error>`) to cast the error type (e.g. `MyCustomError`) to the return type of the function (e.g. `Result<String, io::Error>`)
- `?` can be chained
- `?` can be used in `Result`, `Option`, or anything that inherits `FromResidual`
- Using `?` with `Option<T>` causes `None` to be returned early if expression evals to `None`

Unrecoverable Error
- E.g. `panic!`
- Procedure of unrecoverable error:
  1. Print failure message
  2. Unwind, clean up the stack
  3. quit
- Optionally print stack trace via environment variable
- Optionally *immediate-abort* instead of *unwinding*

### Rust Generics

- Capture type as a paramenter using the syntax `<T, U>` just like conventional OOP languages
- *Trait bounds*: can restrict the generic types to only those that implement certain traits by `<T: SomeTrait>`/`<T: SomeTrait1 + SomeTrait2>`
- Implementing methods on struct:
  - `impl<T> MyStruct<T> {}`: defining type parameter inside `impl` block. `T` can also be trait bounded `impl<T: SomeTrait> MyStruct<T>`
  - `impl MyStruct<i32>`: make type concrete inside `impl` block
  - Methods themselves can also define generic types
  - *Blanket implementations*: `impl<T: MyTrait1> MyTrait2 for MyStruct` - conditionally implement `MyTrait2` only if `T` implements `MyTrait1`
- Rust compiler performs *monomorphization*: the process of turning generic code into specific code by filling in the concrete types that are used

### Rust Traits

- For defining shared behaviour on types (e.g. struct). Similar to interfaces in other languages
- Define trait using e.g. `trait MyTrait {}`. Inside trait, define method signatures e.g. `fn summarize(&self) -> String;`, or define default implementations by giving method a body
- Default implementation can also invoke other not-yet-implemented abstract methods
- Default implementation cannot be invoked by the overriding implementation (unlike other languages that have `super`)
- Implement trait by using the `impl MyTrait for MyStruct {}` syntax
- To use the trait methods, user must also bring the trait into scope. E.g. `use rand::Rng;`
- Restriction that enforces *coherence*: cannot implement external (outside crate) trait on an external type. This is such that outsiders cannot break a working crate by implementing a trait on a type that already implemented that trait
- *Trait bounds*: can force concrete types to those that implement certain traits by `x: impl SomeTrait`/`x: (impl SomeTrait1 + SomeTrait2)` (or `x: &impl SomeTrait`/`x: &(impl SomeTrait1 + SomeTrait2)` if borrowed immutably). Alternative is to use generics
- Define cleaner trait bounds using the `where` syntax

Examples:
- Types can be compared e.g. `>` if they implement `std::cmp::PartialOrd` trait
- Types can be copied if they implement `Copy` trait
- Types will be dropped if they implement `Drop` trait
- Types will be implicitly casted if they implement `From` trait
- `Iterator`
- `Deref` for de-reference coercing
- `Display` and `Debug` for data string representation
- Closures: `FnOnce`, `FnMut`, `Fn`

### Rust Lifetime

- For ensuring references are valid as long as we need them to be
- Every reference has a lifetime. Most of the time, lifetimes are implicit and inferred
- Lifetime annotations: `&'a i32`, `&'a mut i32`
- Capturing lifetime values (ranges) as *generic lifetime parameters* e.g. `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`. `'a` will be a concrete lifetime equal to the intersection of `x` and `y`
- Struct can hold references (every ref must be given a lifetime explicitly), so struct can also take generic lifetime parameters. The whole struct instance becomes invalid when one of its field becomes invalid
- Lifetime elision rules: (rules used by Rust compiler to auto infer lifetimes of references)
  1. Compiler assigns a lifetime parameter to each *input parameter* that’s a reference. If all after all rules are applied, and there is still unassigned *output references*, compiler will throw an error
  2. If there is exactly *1 input* lifetime parameter, that lifetime is assigned to all output lifetime parameters
  3. In the case of *multiple input* lifetime parameters, if 1 of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters
- `'static`: reference can live for the entire duration of the program. E.g. string literals `&str`

### Rust Tests

