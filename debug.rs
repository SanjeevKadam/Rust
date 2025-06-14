Rust Debug Trait Cheat Sheet
🧩 What is Debug?
The Debug trait in Rust allows you to print internal state of values for debugging purposes using the {:?} or {:#?} format specifiers.

{:?} → Compact, single-line format.

{:#?} → Pretty-printed (multi-line and indented).

✅ Basic Example
rust
Copy
Edit
fn main() {
    let age = 30;
    let name = "Alice";
    println!("Debug info: name = {:?}, age = {:?}", name, age);
}
Output:

pgsql
Copy
Edit
Debug info: name = "Alice", age = 30
✅ #[derive(Debug)] for Custom Structs
rust
Copy
Edit
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{:?}", p);   // Debug
    println!("{:#?}", p);  // Pretty Debug
}
Output of {:?}:

css
Copy
Edit
Person { name: "Alice", age: 30 }
Output of {:#?}:

css
Copy
Edit
Person {
    name: "Alice",
    age: 30,
}
🔍 When to Use Debug
Use Debug for development or logging.

Use Display for user-facing output.

⚠️ Important Notes
You must derive or manually implement Debug to use {:?} on custom types.

For most built-in types (String, Vec, HashMap, etc.), Debug is already implemented.

✅ Debug with Complex Types
rust
Copy
Edit
fn main() {
    let numbers = vec![1, 2, 3];
    let map = std::collections::HashMap::from([("a", 1), ("b", 2)]);

    println!("Vector: {:?}", numbers);
    println!("Map: {:#?}", map);
}


fmt::Debug Trait in Rust – Complete Guide
🔸 What is fmt::Debug?
The Debug trait enables formatted output using the {:?} and {:#?} syntax. It’s intended for debugging output rather than end-user display.

🧠 Key Points
All standard library types implement Debug.

Custom types must derive(Debug) to be printed with {:?}.

fmt::Display (user-facing formatting) must be implemented manually.

📌 Example: Debug vs Display
rust
Copy
Edit
// ❌ Cannot print this without manual implementation
struct UnPrintable(i32);

// ✅ Can be printed using {:?}
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    let a = DebugPrintable(42);
    println!("{:?}", a);   // Output: DebugPrintable(42)
    println!("{:#?}", a);  // Pretty print: multi-line
}
🖨️ Pretty Printing with {:#?}
rust
Copy
Edit
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("{:#?}", person);
}
Output:

rust
Copy
Edit
Person {
    name: "Alice",
    age: 30,
}
🧰 Use Cases
Trait	Use Case	Format Symbol
fmt::Debug	Developer debugging/logging	{:?} or {:#?}
fmt::Display	Elegant user-facing output (manual)	{}

🛠 Manual fmt::Display Implementation (Optional)
rust
Copy
Edit
use std::fmt;

struct MyStruct(i32);

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct contains: {}", self.0)
    }
}
📚 Related Concepts
#[derive(Debug)] – Automatically implements Debug.

std::fmt – Formatting traits live here (Display, Debug, Binary, etc.).

attributes – Like derive, allow compiler annotations.

struct – Data structure to implement traits on.

✅ Summary
Feature	Debug	Display
Derivable	✅ Yes	❌ No (manual required)
For Developers	✅ Debugging/logging	❌ (Not ideal)
For End Users	❌ Not elegant	✅ Clean, user-friendly
Syntax	{:?}, {:#?}	{}

--------

Rust Formatting Overview
Rust uses format strings and traits from the std::fmt module to print values in different ways.

🔧 Common Macros
Macro	Description
format!	Returns the formatted string (like String::format)
print!	Prints to standard output (no newline)
println!	Prints to standard output with newline
eprint!	Prints to standard error (no newline)
eprintln!	Prints to standard error with newline

🎨 Formatting Traits and Specifiers
Each formatting style corresponds to a trait:

Trait	Specifier	Example
Display	{}	Human-readable format
Debug	{:?}	Developer/debug-friendly
UpperHex	{:X}	Uppercase hexadecimal
LowerHex	{:x}	Lowercase hexadecimal
Octal	{:o}	Octal format
Binary	{:b}	Binary format
Pointer	{:p}	Memory address format
Scientific	{:e}	Scientific notation
LowerExp	{:e}	e.g. 1.23e+03
UpperExp	{:E}	e.g. 1.23E+03

🧪 Example in Action
rust
Copy
Edit
fn main() {
    let number: u32 = 3735928559;
    
    println!("{}", number);          // Display: 3735928559
    println!("0x{:X}", number);      // UpperHex: 0xDEADBEEF
    println!("0o{:o}", number);      // Octal: 0o33653337357
    println!("{:b}", number);        // Binary: 11011110101011011011111011101111
    println!("{:?}", number);        // Debug: 3735928559
}
🧵 Custom Types
Display must be implemented manually:

rust
Copy
Edit
impl fmt::Display for MyType { ... }
Debug can be auto-derived:

rust
Copy
Edit
#[derive(Debug)]
struct MyType { ... }
