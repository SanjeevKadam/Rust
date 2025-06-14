/*
println! is a macro used to print text to the console.

You write your Rust code in a file like hello.rs.

Use the Rust compiler (rustc) to compile the code:

bash
Copy
Edit
$ rustc hello.rs
$ ./hello
This will run your program and display the output.

println! is a macro (note the !) that writes to the terminal.

rustc compiles your .rs file to a binary executable.

Rustacean is a fun nickname for someone who codes in Rust. 

Let’s dive into how println! works under the hood in Rust. 🧠🦀

🔍 What Is println! in Rust?
println! is not a function — it's a macro.

It’s defined in Rust’s standard library (std), and it's built on top of:

print! macro → which writes to stdout (std::io::stdout()).

format_args! macro → which builds the formatted string safely at compile time.

🧱 How println! is Built
Here’s a simplified version of how println! works:

rust
Copy
Edit
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
✅ Explanation:
If you call println!() with no arguments, it just prints a newline (\n).

If you pass arguments (like "Hello {}", name), it:

Uses format_args! to build the string.

Then uses print! to output that string, plus a newline.

🔗 Chain of Execution
rust
Copy
Edit
println!("Hello, {}!", "Sanjeev");
This expands to something like:

rust
Copy
Edit
print!("{}\n", format_args!("Hello, {}!", "Sanjeev"));
Then behind the scenes:

format_args! builds the format string safely.

print! sends it to std::io::_print, which uses stdout.

💡 Why Use a Macro?
Compile-time safety: format_args! checks your formatting and arguments during compilation.

Flexibility: Macros allow variable arguments (like println!("x={}, y={}", x, y);).

Efficiency: No runtime parsing needed for format strings.

🔒 Bonus: Security and Performance
No string injection issues like in C (printf), because everything is type-checked.

Format string is not a runtime string — it’s compiled into the binary, making it safer and faster.

✅ Final Recap:
println! is a macro that:

Uses format_args! to build the string.

Uses print! to write to standard output with a newline.

It’s compile-time checked, safe, and efficient.

Bonus: Security and Performance
No string injection issues like in C (printf), because everything is type-checked.

Format string is not a runtime string — it’s compiled into the binary, making it safer and faster.
*/
fn main(){
    let x=10;
    let _format = format!("hello, world..!");
    println!("hello, world..! x={}",x);
    println!("hello, world..! x={}",x);
    println!("Hello {}", "Sanjeev");
} 
