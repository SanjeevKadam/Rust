🔸 1. Primitives in Rust
✅ Scalar Types
Type	Description
i8, i16, i32, i64, i128, isize	Signed integers
u8, u16, u32, u64, u128, usize	Unsigned integers
f32, f64	Floating point
char	Unicode character (e.g., 'a')
bool	Boolean: true or false
()	Unit type

✅ Compound Types
Type	Example	Description
Array	[1, 2, 3]	Fixed-size, same-type values
Tuple	(1, true)	Group of mixed-type values

🔸 2. Literals and Operators
Literals: Numbers (1, 0xFF), Chars ('a'), Strings ("hello"), Bools (true)

Underscores: Used for readability → 1_000

Scientific notation: 1e6

Operators: Similar to C-family languages (+, -, *, /, %, etc.)

🔸 3. Custom Types
✅ struct
rust
Copy
Edit
struct Person {
    name: String,
    age: u8,
}
Classic struct: Named fields

Tuple struct: struct Pair(i32, f32);

Unit struct: struct Empty;

Structs can be used as fields in other structs, instantiated with shorthand, and destructured using pattern matching.

✅ enum
rust
Copy
Edit
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}
Represents one of many variants.

Each variant can be a unit, tuple, or struct-like.

Handled using match.

✅ Type Alias
rust
Copy
Edit
type Ops = VeryVerboseEnum;
Used for shorter names in code.

Often used in impl blocks via Self.

🔸 4. Enums as Linked Lists
rust
Copy
Edit
enum List {
    Cons(u32, Box<List>),
    Nil,
}
Recursive structures like linked lists.

Box used for heap allocation.

Custom methods (new, prepend, len, stringify) added via impl.

🔸 5. use Declaration
rust
Copy
Edit
use crate::Stage::{Beginner, Advanced};
use crate::Role::*;
Allows scoped imports so Stage::Beginner → just Beginner.

Reduces repetition and increases readability.

🔸 6. C-like Enums
rust
Copy
Edit
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
}
Enums with assigned numeric values (discriminants).

Can be cast to integers: Color::Red as i32

🔸 7. Constants
✅ const
rust
Copy
Edit
const THRESHOLD: i32 = 10;
Immutable.

Must be annotated.

Available at compile time.

✅ static
rust
Copy
Edit
static LANGUAGE: &str = "Rust";
Global variables with 'static lifetime.

mut static requires unsafe to access.

🔸 8. Variable Bindings (Rust by Example 4.1 – 4.4)
Feature	Example	Notes
Immutable	let x = 5;	Default in Rust
Mutable	let mut x = 5; x = 6;	Use mut to allow changes
Shadowing	let x = 5; let x = x + 1;	Redefines the same variable
Scope	Inner blocks can shadow outer values	Scoped variable lifetime
Declare-First	let x; x = 5;	Must initialize before use

🧠 Key Concepts Reinforced
Concept	Rust's Behavior
Safety	No uninitialized variables or nulls
Memory Ownership	Variables "own" their data by default
Immutability	Enforced unless explicitly made mutable
Type Inference	Strong, but annotations help clarity & control
Pattern Matching	Powerful with match and destructuring


// =======================================================
// Rust Concepts Summary with Examples
// =======================================================

// ---------- 1. PRIMITIVE TYPES ----------
let int_num: i32 = 42;
let float_num: f64 = 3.14;
let is_active: bool = true;
let letter: char = 'R';
let unit: () = (); // unit type

// ---------- 2. STRUCTS ----------
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Pair(i32, f64); // Tuple struct
struct Unit; // Unit struct

fn structs_example() {
    let peter = Person { name: String::from("Peter"), age: 27 };
    println!("{:?}", peter);

    let pair = Pair(1, 0.5);
    println!("Pair: {}, {}", pair.0, pair.1);

    let _unit = Unit;
}

// ---------- 3. ENUMS ----------
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Click { x, y } => println!("Clicked at: {}, {}", x, y),
    }
}

// ---------- 4. TYPE ALIAS ----------
enum Operation {
    Add,
    Subtract,
}

type Ops = Operation;

impl Operation {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// ---------- 5. LINKED LIST WITH ENUM ----------
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        List::Nil
    }

    fn prepend(self, val: u32) -> Self {
        List::Cons(val, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            List::Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => "Nil".to_string(),
        }
    }
}

// ---------- 6. USE DECLARATION ----------
enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn role_example() {
    use Stage::*;
    use Role::*;

    let stage = Beginner;
    let role = Teacher;

    match stage {
        Beginner => println!("Starting journey..."),
        Advanced => println!("Advanced learner!"),
    }

    match role {
        Student => println!("Learning..."),
        Teacher => println!("Teaching..."),
    }
}

// ---------- 7. C-LIKE ENUM ----------
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn color_enum() {
    println!("Red is #{:06x}", Color::Red as i32);
}

// ---------- 8. CONSTANTS ----------
const THRESHOLD: i32 = 10;
static LANGUAGE: &str = "Rust";

fn const_example(n: i32) {
    println!("LANGUAGE: {}", LANGUAGE);
    println!("THRESHOLD: {}", THRESHOLD);
    println!("{} is {}", n, if n > THRESHOLD { "big" } else { "small" });
}

// ---------- 9. VARIABLE BINDINGS (4.1 - 4.4) ----------
fn variable_bindings() {
    let x = 5;
    let mut y = 10;
    y = 20;

    let z = 30;
    let z = z + 1; // shadowing

    let a;
    a = 50;

    {
        let a = 100; // inner scope shadowing
        println!("Inner a: {}", a);
    }
    println!("Outer a: {}", a);
}

// ---------- MAIN ----------
fn main() {
    structs_example();

    inspect(WebEvent::PageLoad);
    inspect(WebEvent::KeyPress('A'));
    inspect(WebEvent::Click { x: 10, y: 20 });

    let op = Ops::Add;
    println!("Ops result: {}", op.run(3, 2));

    let mut list = List::new();
    list = list.prepend(1).prepend(2).prepend(3);
    println!("List length: {}", list.len());
    println!("List: {}", list.stringify());

    role_example();
    color_enum();
    const_example(16);
    variable_bindings();
}
