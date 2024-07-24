// --------------------------------------
//           Program Outputs
//           Comments and Their Styles
// --------------------------------------

fn main() {
    // This is the first program in this tutorial.
    // Here is another line of comment.

    /* 
       This comment spans 
       multiple lines 
       and demonstrates a block comment.
    */

    // Printing a simple message.
    println!("Hello from the Rust program!");

    // Printing with an inline comment affecting the command.
    print/*ln*/!("Hello, world without newline!");

    // Demonstrating basic output commands.
    println!("The value of the constant is {}", 100);

    // Printing a formatted string with placeholders.
    println!("My first name is {} and my last name is {}", "Jane", "Doe");

    // Using the `print!` command which does not add a newline.
    print!("This text is printed ");
    print!("on the same line.");

    // Printing text over multiple lines.
    print!("\nThis text is split
            across multiple 
            lines."); 

    // Using escape sequences for special formatting.
    println!("\n\\n\n This line starts after a double newline. \t This line has a tab.");

    // Demonstrating the use of backslashes.
    println!("Printing single quote \' and double quote \"");
    println!("Printing a single backslash \\");
    print!("This text is overwritten \r only this part is shown");

    // Using positional arguments in the print macro.
    println!("\nI have been {2} for {1} years and I {0} it", "enjoying", 15, "coding");

    // Using named arguments in the print macro.
    println!("{language} is a systems programming language that's great for {activity}.", language="Rust", activity="development");

    // Performing and printing basic arithmetic.
    println!("The result of 42 + 58 is {}", 42 + 58);
}
