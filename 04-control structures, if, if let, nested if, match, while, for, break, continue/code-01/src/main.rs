fn main() {
    // -------------------------------------------
    //           Conditional Statements 
    //           - Basic If 
    //           - If with Compound Conditions
    //           - If-Else 
    //           - If-Else If Ladder 
    // -------------------------------------------

    // 1. Basic if statement 
    /* General Structure:

    if condition {
        // code to execute if condition is true
    }
    */ 

    // Example 
    /*
    let value = 25;
    if value < 30 {
        println!("The value is less than 30");
    }
    println!("This line runs regardless of the above condition");
    */ 
    
    /*
    let score = 75;
    if score >= 70 && score <= 80 {
        println!("The score is within the expected range");
    }
    */ 

    /*
    let is_active = true;
    let is_admin = false;

    if is_active == true || is_admin == true {
        println!("At least one condition is true");
    }

    let is_logged_in = true;
    if is_logged_in != false {
        println!("This will run if the user is logged in");
    }
    */ 

    /*
    let condition_1 = true;
    let condition_2 = false;
    let limit = 40;

    // condition_1 = true && (condition_2 = false || limit < 30) and 
    // (condition_1 = true && condition_2 = false) || limit < 30

    if condition_1 == true && condition_2 == false || limit < 30 {
        println!("This part executes based on the above condition");
    }
    */

    // 3. If-Else Conditions 
    /* General Structure:

    if condition {
        // code to execute if condition is true
    }
    else {
        // code to execute if condition is false
    }
    */ 

    /*
    let score = 90;
    if score > 60 {
        println!("You have passed the test");
    } else {
        println!("You have failed the test");
    }
    */

    /* General Structure:

    if condition {
        // code to execute if condition is true
    }
    else if condition_2 {
        // code to execute if condition_2 is true
    }
    else if condition_3 { 
        // code to execute if condition_3 is true
    }
    else {   // optional
        // code to execute if none of the above conditions are true
    }
    */

    let test_score = 85;
    let mut grade = 'U';
    if test_score >= 80 {
        grade = 'A';
    } else if test_score >= 70 {
        grade = 'B';
    } else if test_score >= 60 {
        grade = 'C';
    } else if test_score >= 50 {
        grade = 'D';
    } else {   // Removing the else block is possible but leaves no default action
        grade = 'E';
    }

    println!("The assigned grade is {}", grade);
}
