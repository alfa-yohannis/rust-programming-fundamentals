fn main() {
    // Initialize a string to check for palindrome
    let input = String::from("abbbbaa");
    let mut is_palindrome = true; 

    // Check if the string is empty
    if input.len() == 0 {
        is_palindrome = true;
        println!("\n\nThe input is palindrome {:?}", is_palindrome);
        return;
    }

    // Initialize indices for comparison
    let mut last = input.len() - 1;
    let mut first = 0;

    // Convert string to bytes for comparison
    let my_vec = input.as_bytes();

    // Check characters from both ends
    while first < last {
        if my_vec[first] != my_vec[last] {
            is_palindrome = false;
            break; // Exits loop if characters don't match
        }

        // Move towards the center
        first += 1;
        last -= 1;
    }

    // Print result
    println!("\n\nThe input is palindrome {:?}", is_palindrome);
}
