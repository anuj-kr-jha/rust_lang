// naming convention : [snake_case] => lower cased world separated with underscore
// specifying type is optional
// we end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin
// if the function contain only on expression we can skip semicolon
// variable and references (&) are immutable by default, we need mto add 'mut' keyword to make it mutable

// main is entry point for program
fn main() {
    let x: i32 = 5; // immutable: value can't change
    let mut y = 50; // mutable: value can be changed
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    y = 6;
    println!("Now, The value of y is: {y}"); // invoking macro println. '!' is mandatory for macro invocation
    println!("x = {x}, y = {y} and y+2 = {}", y + 2);
    another_function(); // invoking function another_function

    user_input_function();
}

//
fn another_function() {
    print!("I'm another function\n");
}

/*
  cargo fmt : This commands format the entire code
  cargo check : This command quickly checks your code to make sure it compiles but doesn’t produce an executable
  cargo build : It build the code and generate un-optimized executables. Mostly used for developments
  cargo build --release : It build the code and generate optimized executables
  cargo run : This program
*/

/*
  The prelude is the list of things that Rust automatically imports into every Rust program
  It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.
*/

fn user_input_function() {
    // use std::io; // we can also skip this and then in that case we need to prefix io with 'std::' every where

    println!("Please input something");

    // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    let mut input: String = String::new();

    std::io::stdin() // Constructs a new handle to the standard input of the current process.
        .read_line(&mut input) // Locks this handle and reads a line of input, appending it to the specified buffer.
        .expect("Failed to read line");

    /*
        read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
    */

    /*
        we can also write it in one line like
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    */

    println!("You entered: {input}");
}
