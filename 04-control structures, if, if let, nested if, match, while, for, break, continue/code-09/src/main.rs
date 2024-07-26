fn main() {
    // Read user input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let n: i32 = input.trim().parse().expect("Invalid input");
    
    // Initialize vectors to store multiples of 3 and 5
    let mut multiples_of_3 = vec![0];  // Using Vec instead of array due to dynamic size
    let mut multiples_of_5 = vec![0];  
    
    // Populate vectors with 1 if the index is a multiple of 3 or 5, otherwise 0
    for i in 1..n {
        if i % 3 == 0 { multiples_of_3.push(1); } else { multiples_of_3.push(0); }
        if i % 5 == 0 { multiples_of_5.push(1); } else { multiples_of_5.push(0); }
    } 
    
    // Combine the multiples of 3 and 5 into a single list
    let mut combined_list = vec![0]; 
    for i in 1..n as usize {
        if multiples_of_3[i] == 1 || multiples_of_5[i] == 1 {
            combined_list.push(1);
        } else {
            combined_list.push(0);
        }
    }

    // Compute the values of multiples and store them in a vector
    let mut values_of_multiples: Vec<i32> = vec![0]; 
    for i in 1..=n {
        values_of_multiples.push(combined_list[i as usize] * i); 
    }

    // Print the multiples and their sum
    println!("\n\nMultiples of 3 and 5 are = {:?}", values_of_multiples);  
    println!("\n\nThe sum of the multiples is = {:?}", values_of_multiples.iter().sum::<i32>());  
}
