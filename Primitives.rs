🧱 Rust Primitives — Summary & Examples
Rust provides a rich set of primitive types used to build all higher-level structures in a performant and memory-safe way.

📌 1. Scalar Types (Single value)
Type	Description	Example
i8..i128	Signed integers	let x: i32 = -42;
u8..u128	Unsigned integers	let y: u8 = 200;
isize	Signed pointer-sized integer	Platform-dependent
usize	Unsigned pointer-sized integer	Platform-dependent
f32/f64	Floating-point (32/64 bit)	let pi = 3.14;
char	Unicode scalar (4 bytes)	'a', '∞', '中'
bool	Boolean values	true, false
()	Unit type (only one value: ())	Used in empty returns

📌 2. Compound Types
Type	Description	Example
Tuple	Fixed-size collection of mixed types	let tup = (1, true, 'x');
Array	Fixed-size list of same type	let arr = [1, 2, 3, 4];
Slice	Dynamically sized view of a collection	let slice = &arr[1..3];

🧠 Tuple Example
rust
Copy
Edit
let tup: (i32, f64, char) = (500, 6.4, '🦀');
let (x, y, z) = tup;
println!("Values: {}, {}, {}", x, y, z);
🧠 Array & Slice Example
rust
Copy
Edit
let arr: [i32; 3] = [10, 20, 30];
let slice: &[i32] = &arr[1..];
println!("Second element: {}", slice[0]); // 20
📌 3. Literals & Number Formats
Rust supports many ways to represent values directly:

Type	Example	Notes
Integer	42, 0xff, 0o77, 0b1010	Decimal, Hex, Octal, Binary
Float	3.14, 1e3	Supports E-notation
Char	'a', '∞', '中'	Unicode scalar
String	"hello"	Double quotes
Boolean	true, false	Logical values
Unit	()	No meaningful value

📌 Use suffixes like 10u32, -5i64, or 3.14f32 to explicitly define the type.

📌 4. Operators
Rust supports familiar C-like operators:

Category	Operators
Arithmetic	+, -, *, /, %
Comparison	==, !=, <, >, <=, >=
Logical	&&, `
Bitwise	&, `
Assignment	=, +=, -=, etc.

Precedence is similar to C; parentheses can help clarify order of execution.

📌 Type Inference & Annotations
Rust infers types when possible:

rust
Copy
Edit
let x = 10;  // i32 by default
let y = 1.5; // f64 by default
You can explicitly annotate:

rust
Copy
Edit
let x: u32 = 1000;
let pi: f32 = 3.14;
✅ Quick Recap Table
Feature	Key Points
Scalar types	i32, f64, bool, char, ()
Compound	Tuple, Array, Slice
Literals	Numbers, Unicode chars, strings, etc.
Operators	Similar to C: arithmetic, comparison, logic
Inference	Defaults: i32 for integers, f64 for floats
