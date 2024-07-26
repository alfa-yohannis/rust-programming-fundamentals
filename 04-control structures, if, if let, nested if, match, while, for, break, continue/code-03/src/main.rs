fn main() {
    // -------------------------------------------
    // Matching in Rust
    // - Simple match
    // - Converting if-else ladder to match
    // - Using match in place of if let
    // -------------------------------------------

    /* General syntax for match:

       match variable {
           pattern1 => { actions },
           pattern2 => { actions },
           pattern3 => { actions },
           _ => { default actions },
       }
    */    

    /*
    let value = 30; 
    match value {
        1 | 2 => println!("Value is 1 or 2"),  
        3 | 4 => println!("Value is 3 or 4"), // Single pipe for OR
        5..=40 => println!("Value is between 5 and 40 inclusive"),       
        _ => println!("Value is greater than 40"), 
    }
    */

    /*
    let points = 72; 
    let mut level = 'U'; 

    match points {
        90..=100 => level = 'X', 
        80..=89  => level = 'Y', 
        70..=79  => level = 'Z',
        60..=69  => level = 'W',
        _ =>       level = 'V',
    }
    println!("Achieved level is {}", level);
    */

    /* 
    let result = match condition {
        pattern1 => { actions },
        pattern2 => { actions },
        pattern3 => { actions },
        _ => { default actions },
    };
    */

    let points = 85; 
    let level = match points {
        90..=100 => 'X', 
        80..=89  => 'Y', 
        70..=79  => 'Z',
        60..=69  => 'W',
        _ => 'V',
    };
    println!("Achieved level is {}", level);
}
