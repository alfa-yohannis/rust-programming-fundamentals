fn main() {
    // -------------------------------------------
    // Loops in Rust
    // - Infinite loops
    // - While loops
    // -------------------------------------------

    /*
    loop {
        println!("This loop runs forever");
    }
    */

    /*
    let secret_num = 7;
    println!("Guess the number between 1 and 20");
    let mut correct = false;

    while !correct {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let input: u8 = input.trim().parse().expect("Invalid input");
        
        if secret_num == input {
            println!("You guessed it!");
            correct = true;
        } else {
            println!("Try again!");
        }
    }
    */

    /*
    println!("Enter a number, and I'll find the next number 
             divisible by both 2 and 5");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let mut input: u8 = input.trim().parse().expect("Invalid input");
    let mut found = false;

    while !found {
        input += 1;
        if input % 2 == 0 && input % 5 == 0 {
            println!("The next number divisible by both 2 and 5 is {}", input);
            found = true;
        }
    }
    */

    println!("Enter a number, and I'll find the next number divisible by both 2 and 5");

    let mut num = String::new();
    std::io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input.");
    let mut num: u8 = num.trim().parse().expect("Invalid input");

    num += 1;
    while !(num % 2 == 0 && num % 5 == 0) {
        num += 1;
    }
    println!("The next number divisible by both 2 and 5 is {}", num);
}
