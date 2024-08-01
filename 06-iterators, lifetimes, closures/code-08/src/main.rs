// Example: Sum of Multiples of 3 and 5
// Refined version using iterators

fn main() {
    // Read an integer from user input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let limit: u32 = input.trim().parse().expect("Invalid input");

    // Filter numbers that are multiples of 3 or 5 and collect them
    let multiples = (1..limit)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .collect::<Vec<u32>>();

    // Print the filtered multiples and their sum
    println!("Multiples of 3 or 5: {:?}", multiples);
    println!("Sum of multiples: {:?}", multiples.iter().sum::<u32>());
}
