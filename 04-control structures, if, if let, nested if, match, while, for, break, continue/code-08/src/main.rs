fn main() {
    // -------------------------------------------
    // Calculating Differences in Sums and Squares
    // - Input a number from the user
    // - Calculate square of the sum and sum of the squares
    // - Compute the difference
    // -------------------------------------------

    // Read user input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    
    let n: i32 = input.trim().parse().expect("Invalid input");

    // Initialize variables for calculations
    let mut sum_of_numbers = 0;  
    let mut sum_of_squares = 0; 

    // Loop through numbers from 1 to n
    for i in 1..=n { 
        sum_of_numbers += i;   
        sum_of_squares += i.pow(2);  
    }

    // Calculate the square of the sum and the difference
    let difference = sum_of_numbers.pow(2) - sum_of_squares; 

    // Print the result
    println!("The difference between the square of the sum and the sum of squares for N = {} is {}", n, difference);  
}
