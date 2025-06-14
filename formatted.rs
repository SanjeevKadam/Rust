Rust Formatted Print Cheat Sheet
📌 1. Common Formatting Macros in std::fmt
Macro	Description
format!	Returns formatted text as a String.
print!	Prints text to stdout (no newline).
println!	Prints text to stdout, ends with newline.
eprint!	Prints text to stderr (no newline).
eprintln!	Prints text to stderr, ends with newline.

📌 2. Basic and Advanced Usage of println!
✅ Basic Placeholder
rust
Copy
Edit
println!("{} days", 31);         // Output: 31 days
✅ Positional Arguments
rust
Copy
Edit
println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
// Output: Alice, this is Bob. Bob, this is Alice
❌ Incorrect Positional Usage
rust
Copy
Edit
// println!("My name is {0}, {1} {0}", "Bond");  // ❌ Error
println!("My name is {0}, {1} {0}", "Bond", "James");
// ✅ Output: My name is Bond, James Bond
✅ Named Arguments
rust
Copy
Edit
println!("{subject} {verb} {object}",
         object="the lazy dog",
         subject="the quick brown fox",
         verb="jumps over");
📌 3. Format Specifiers
🔢 Numeric Base Formatting
rust
Copy
Edit
println!("Base 2: {:b}", 69420);   // 10000111100101100
println!("Base 8: {:o}", 69420);   // 207454
println!("Base 16: {:x}", 69420);  // 10f2c
📏 Width, Padding, and Alignment
rust
Copy
Edit
println!("{number:>5}", number=1);     // "    1"
println!("{number:0>5}", number=1);    // "00001"
println!("{number:0<5}", number=1);    // "10000"
📐 Dynamic Width
rust
Copy
Edit
let number = 1;
let width = 5;
println!("{number:0>width$}");         // "00001"
📌 4. Variable Capturing (Rust 1.58+)
rust
Copy
Edit
let number = 1.0;
let width = 5;
println!("{number:>width$}");          // "    1"
📌 5. Printing Custom Types with fmt::Display
rust
Copy
Edit
use std::fmt;

struct Structure(i32);

// Implement fmt::Display for Structure
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure({})", self.0)
    }
}

fn main() {
    println!("This struct `{}` will print!", Structure(3));
}
📌 6. Format Decimal Places
rust
Copy
Edit
let pi = 3.141592;
println!("Pi is roughly {:.3}", pi);  // Output: Pi is roughly 3.142
✅ Fixed and Enhanced Main Example
rust
Copy
Edit
fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("Base 10:               {}",   69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);

    // ✅ Fixed version of the positional arguments
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    use std::fmt;

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Structure({})", self.0)
        }
    }

    println!("This struct `{}` will print!", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
