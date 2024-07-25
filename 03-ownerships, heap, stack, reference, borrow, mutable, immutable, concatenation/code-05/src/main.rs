fn main() {   
    // -------------------------------------------
    // 		String Concatenation and Ownership
    // -------------------------------------------

    /*
    let greeting = String::from("hi"); 
    let target: &str = "there";
    
    let combined = greeting + target;  // Ownership transferred here
    println!("{}", combined);
    */

    /*
    let greeting = String::from("hi"); 
    let place = String::from("there");
    
    let combined = greeting + &place;  // Ownership of only greeting changed
    println!("{} {}", combined, place); 
    */

    let greeting = String::from("hi"); 
    let place = String::from("there");
    let addition = String::from(" from Rust");

    let combined = greeting + &place + &addition;  // Ownership of only greeting changed
    println!("{} {} {}", combined, place, addition); 
}
