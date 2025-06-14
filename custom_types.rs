Rust Custom Types — struct, enum, const, static
Rust lets you define your own types using:

✅ struct – to define structured data

✅ enum – to define variants or choices

✅ const / static – for fixed constant values

🧩 1. struct — Structures
A struct is used to group related data into a single type.

🧠 Example: Define and Use a Struct
rust
Copy
Edit
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{} is {} years old.", p.name, p.age);
}
📌 Struct Types:
Named Fields: struct Person { name: String, age: u8 }

Tuple Structs: struct Color(u8, u8, u8);

Unit Structs: struct Marker; (zero-size)

🔄 2. enum — Enumerations
An enum lets you define a type that can be one of several variants.

🧠 Example: Enum with Pattern Matching
rust
Copy
Edit
enum Direction {
    North,
    East,
    South,
    West,
}

fn go(dir: Direction) {
    match dir {
        Direction::North => println!("Going north!"),
        Direction::East => println!("Going east!"),
        Direction::South => println!("Going south!"),
        Direction::West => println!("Going west!"),
    }
}
📌 Enum Variants with Data
rust
Copy
Edit
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    // Handle with match...
}
🔒 3. const vs static
Feature	const	static
Value	Immutable compile-time constant	Immutable or mutable global value
Scope	Local to current scope	Global across program
Thread	Not shared	static mut can be unsafe/shared
Memory	Inlined	Stored in static memory segment

✅ Examples:
rust
Copy
Edit
const MAX_POINTS: u32 = 100_000; // Inlined wherever used

static PI: f64 = 3.1415926535;   // Stored in program's memory
⚠️ static mut is unsafe:
rust
Copy
Edit
static mut COUNTER: u32 = 0;

unsafe {
    COUNTER += 1;
}
✅ Summary Table
Keyword	Purpose	Example Use Case
struct	Group named fields	User profile, Point2D
enum	Define variant types	Result, Option, State machine
const	Immutable constant values	App-wide constants
static	Program-wide static memory	Shared settings or counters
------------------

Rust struct – Structures in Rust
Rust supports three types of structures, each useful in different scenarios:

🧩 1. Classic Structs (Named Fields)
These are like C-style structs and are used when you want named fields.

rust
Copy
Edit
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
✅ Usage
rust
Copy
Edit
let person = Person {
    name: String::from("Peter"),
    age: 27,
};
println!("{:?}", person);
🎯 2. Tuple Structs
These behave like tuples but have a named type, helpful when types are the same but semantically different.

rust
Copy
Edit
struct Pair(i32, f32);

let pair = Pair(1, 0.1);
println!("pair.0 = {}, pair.1 = {}", pair.0, pair.1);

let Pair(x, y) = pair;
println!("Destructured: {} and {}", x, y);
⚙️ 3. Unit Structs
These have no fields at all. Used for marker traits, zero-sized types, or generics.

rust
Copy
Edit
struct Unit;
let _marker = Unit;
🧠 Advanced Usage in Your Example:
📌 Nested Structs
rust
Copy
Edit
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
Here, one struct (Rectangle) contains other structs (Point) as fields—useful for building complex types.

📌 Struct Update Syntax
rust
Copy
Edit
let bottom_right = Point { x: 10.3, ..another_point };
Rust allows copying remaining fields from an existing struct. Here, y is copied from another_point.

📌 Destructuring Structs
rust
Copy
Edit
let Point { x: left_edge, y: top_edge } = point;
You can pattern match structs directly into variables.

🔄 Recap Table
Type	Syntax	Purpose
Classic Struct	struct Name { field: Type }	Named fields
Tuple Struct	struct Name(Type1, Type2)	Positional fields with a name
Unit Struct	struct Name;	Marker types, generics, ZSTs

✅ Takeaways
Use #[derive(Debug)] to enable println!("{:?}", ..)

Structs can be nested

Fields can be destructured and updated

Tuple structs give semantic meaning to positional data

Unit structs are lightweight and compile to zero bytes

--------------------

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

---------------


🧾 use Declaration
The use keyword allows importing names from modules or enums so you don’t have to keep qualifying them:

✅ Without use:
rust
Copy
Edit
let stage = Stage::Beginner;
✅ With use:
rust
Copy
Edit
use crate::Stage::{Beginner, Advanced};
let stage = Beginner;
Useful when:

You’re calling variants frequently.

You want cleaner match statements.

🎨 Enum as C-like Enums
Enums can act like constants and have underlying integer values.

Example:
rust
Copy
Edit
enum Number {
    Zero,  // implicitly 0
    One,   // implicitly 1
    Two,   // implicitly 2
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
rust
Copy
Edit
println!("zero is {}", Number::Zero as i32); // Output: zero is 0
🔗 Linked List Using Enum
Enums allow building recursive data types like linked lists.

Enum Structure:
rust
Copy
Edit
enum List {
    Cons(u32, Box<List>),  // value + pointer
    Nil                    // end of list
}
Method Implementations:
rust
Copy
Edit
impl List {
    fn new() -> List { Nil }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}
Output Example:
text
Copy
Edit
linked list has length: 3
3, 2, 1, Nil
📌 Constants in Rust
Rust has two kinds of compile-time constants:

Keyword	Description	Mutable	Scope
const	Immutable constant	❌ No	Any scope
static	Global variable with static lifetime	✅ Yes*	Global only

rust
Copy
Edit
static LANGUAGE: &str = "Rust";       // Static
const THRESHOLD: i32 = 10;            // Constant

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}
⚠️ static mut is allowed but requires unsafe access because of data races in multithreading.

🧠 TL;DR Summary
Concept	Use Case
use	Avoid prefixing with enum/module names
Enum as C-like	Numeric constants with readable names
Enums	Create flexible types, pattern matching
Constants	Define global and scoped fixed values
Linked List	Recursive enum structure using Box
