fn main() {

    // -------------------------------------------------
    //           Strings
    //            - Fixed length strings (&str)
    // -------------------------------------------------

    // Declare a fixed-length string.
    let fixed_str = "Immutable fixed length string"; 
    println!("The text in the fixed string is \"{}\" ", fixed_str); 

    // -------------------------------------------------
    //           Strings
    //            - Variable length strings
    // -------------------------------------------------

    // Declare a mutable, growable string.
    let mut dynamic_str = String::from("This string can expand");  
    println!("The initial text in the dynamic string is \"{}\" ", dynamic_str); 

    // -------------------------------------------------
    //           Strings
    //            - Adding and removing characters/strings
    // -------------------------------------------------

    // Add a character to the dynamic string.
    dynamic_str.push('!');
    println!("After adding a character, the string is \"{}\" ", dynamic_str); 

    // Remove the last character from the dynamic string.
    dynamic_str.pop(); 
    println!("After removing the last character, the string is \"{}\" ", dynamic_str); 

    // Append a substring to the dynamic string.
    dynamic_str.push_str(" that can grow and shrink");
    println!("After adding more text, the string is \"{}\" ", dynamic_str); 

    // -------------------------------------------------
    //           Strings
    //            - Functions on strings
    // -------------------------------------------------

    // Demonstrate various string functions.
    println!(
        "String details:\n- Is the string empty? {}\n- Length of the string: {}\n- String capacity: {}\n- Does the string contain 'grow'? {}",
        dynamic_str.is_empty(),
        dynamic_str.len(),
        dynamic_str.capacity(),
        dynamic_str.contains("grow")
    );

    // Add spaces and demonstrate trimming.
    dynamic_str.push_str("   "); 
    println!(
        "Length before trimming: {}, Length after trimming: {}",
        dynamic_str.len(),
        dynamic_str.trim().len()
    ); 

    // Convert a number to a string and compare.
    let number = 42;
    println!("The number as a string: {}", number.to_string()); 
    println!("Does the string equal '42'? {}", number.to_string() == "42"); 

    // Convert a character to a string and compare.
    let character = 'b'; 
    println!(
        "The character as a string: {}, Is it equal to 'b'? {}",
        character.to_string(),
        character.to_string() == "b"
    );  

    // Create a string from a name.
    let full_name = "John Doe".to_string(); 
    println!("This string contains a name: {}", full_name);

    // -------------------------------------------------
    //           Strings
    //            - Creating an empty string
    // -------------------------------------------------

    // Create an empty string and check its length.
    let empty_str = String::new();
    println!("Length of the empty string: {}", empty_str.len()); 

    // -------------------------------------------------
    //           Strings
    //            - Formatting and concatenating strings
    // -------------------------------------------------

    // Using `format!` to combine strings.
    let first_name = "John".to_string();
    let last_name = "Doe".to_string();
    let full_intro = format!("My first name is {}, and my last name is {}", first_name, last_name);
    println!("{}", full_intro);

    // Concatenating strings using `format!`.
    let part_1 = String::from("Hello");
    let part_2 = String::from(", World!"); 
    let combined_str = format!("{}{}", part_1, part_2);
    println!("The combined string is \"{}\"", combined_str);

}
