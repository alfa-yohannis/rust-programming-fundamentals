fn main() {

    // -------------------------------------------------
    // Initializing Multiple Variables
    // -------------------------------------------------

    // Initialize height and weight variables.
    let (height, weight) = (175, 70.5);
    println!("Height: {} cm, Weight: {} kg", height, weight);

    // -------------------------------------------------
    // Readability of Large Numbers
    // -------------------------------------------------

    // Declare city population with underscores for readability.
    let population = 8_000_000;
    println!("City population: {}", population);

    // -------------------------------------------------
    // Integer Overflow
    // -------------------------------------------------
    
    // Uncommenting the following line would cause an overflow error for `u8`.
    // let max_byte: u8 = 300;
    
    // -------------------------------------------------
    // Decimal Numbers in Different Formats
    // -------------------------------------------------

    // Declare a distance variable and print it in different numeral systems.
    let distance = 123;
    println!("Distance: Hexadecimal {:X}, Octal {:o}, Binary {:b}", distance, distance, distance);

    // -------------------------------------------------
    // Snake Case Convention for Variables
    // -------------------------------------------------

    // Naming convention using snake_case for variables.
    let student_count = 30; // Preferred over let StudentCount = 30;

    // -------------------------------------------------
    // Operations on Numbers in Different Formats
    // -------------------------------------------------

    // Demonstrate type casting and operations on variables.
    let apples = 20;
    let price_per_kg = 2.75;
    let total_price = price_per_kg as i32 * apples;
    println!("Total price for apples: {}", total_price);

    // -------------------------------------------------
    // Shadowing
    // -------------------------------------------------

    // Case 1: Simple Shadowing
    println!("\n***************************************************** \n");

    // Shadowing allows redeclaration of a variable.
    // let rate = 10;
    // let rate = rate * 2;
    // println!("Updated rate: {}", rate);

    // Case 2: Shadowing with `mut`. It works similarly.
    // let mut temperature = 25;
    // let temperature = temperature - 5;
    // println!("Adjusted temperature: {}", temperature);

    // Case 3: Changing the type through shadowing.
    /* let value = 100;
    println!("Value as integer: {}", value);
    let value = 'B';
    println!("Value as character: {}", value);
    let value = 22.9;
    println!("Value as float: {}", value);
    */

    // Case 4: Shadowing within code blocks.
    let mut grade = 88;

    {
        grade = 90; // Modifying variable inside a block.
        println!("Grade inside block: {}", grade);
    }
    println!("Grade outside block: {}", grade);

    // -------------------------------------------------
    // Constants
    // -------------------------------------------------

    // Define a constant with uppercase and underscores.
    const MAX_CAPACITY: u32 = 50_000;
    println!("Maximum capacity: {}", MAX_CAPACITY);
}
