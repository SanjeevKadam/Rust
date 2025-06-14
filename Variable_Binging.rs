Variable Bindings in Rust – Summary
Rust uses variable bindings to assign names to values. These bindings control immutability, mutability, scope, and shadowing.

🔹 1. Basic Variable Binding
rust
Copy
Edit
fn main() {
    let x = 5;
    println!("x is: {}", x);
}
let creates an immutable variable by default.

You cannot change x after this without mut.

🔹 2. Mutability
rust
Copy
Edit
fn main() {
    let mut x = 5;
    x = 10;
    println!("x is now: {}", x);
}
mut allows a variable to be modified.

Without mut, any assignment after initialization will result in a compiler error.

🔹 3. Scope and Shadowing
rust
Copy
Edit
fn main() {
    let x = 5;

    {
        let x = 12; // shadows outer x
        println!("inner x: {}", x);
    }

    let x = x * 2; // shadows outer x with a new value
    println!("outer x: {}", x);
}
Inner scopes can shadow variables.

Shadowing creates a new variable, not a mutation.

Original value remains unchanged outside the scope.

🔹 4. Declare First (4.4)
rust
Copy
Edit
fn main() {
    let x;
    x = 42;
    println!("x: {}", x);
}
You can declare a variable without assigning a value.

You must assign a value before using it, or Rust will throw a compile-time error.

✅ Summary Table
Feature	Description
let	Declares a new variable (immutable by default)
mut	Makes a variable mutable
Shadowing	Allows reuse of the same name with a new value
Scope	Variables follow block-level scoping
Declare-first	You can declare without a value, but must assign before use
