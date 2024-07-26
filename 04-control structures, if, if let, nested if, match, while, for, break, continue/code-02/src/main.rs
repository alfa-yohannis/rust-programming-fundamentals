fn main() {
    // -------------------------------------------
    // Conditional Statements 
    // - Nested If 
    // - If Let 
    // - If Let in If-Else Ladder 
    // - Match Statement 	
    // -------------------------------------------

    /* 
    if outer_check {
        // Executes if outer_check is true 
        if inner_check {
            // Executes if both outer_check and inner_check are true 
        } else { 
            // Executes if outer_check is true but inner_check is false
        }
    } else {
        // Executes if outer_check is false
    }
    */  

    // Example 
    /*
    println!("Please input a number"); 
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");
    let input: i32 = input.trim().parse().expect("Invalid number");

    // Outer check 
    if input != 0 {
        // Inner check
        if input % 2 == 0 {
            println!("The number is even.");
        } else {
            println!("The number is odd.");
        }  
    } else {
        println!("The number is zero, which is neither even nor odd.");
    }
    */

    // 2. If let 
    /*
    let result = if condition {
        // Executes if condition is true and returns a value 
    } else {
        // Executes if condition is false and returns a value 
    }; // Semicolon required
    */

    /*
    let num = if false {
        10
    } else {
        20 // Both branches must return the same type
    }; 
    println!("Result = {}", num); 
    */

    let score = 75; 

    let rank = if score >= 85 {  
        'A'
    } else if score >= 75 {
        'B'
    } else if score >= 65 { 
        'C'
    } else if score >= 55 {
        'D'
    } else {
        'F' // Else block is mandatory
    };

    println!("The achieved rank is {}", rank);
}
