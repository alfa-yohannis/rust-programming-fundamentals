fn main() {
    // -------------------------------------------
    // Break: Stopping a loop early
    // Continue: Skipping the rest of the current iteration
    // -------------------------------------------

    /*
    let mut num = 100;
    loop {
        num -= 1;
        if num % 13 == 0 {
            break;
        }
    }
    println!("The highest number less than the given number divisible by 13 is {}", num);
    */
    
    /*
    let mut num = 0;
    let mut count = 0;
    loop {
        num += 1;
        if num % 5 == 0 && num % 3 == 0 {
            println!("\nThe number divisible by both 3 and 5 is: {}\n", num);
            count += 1;
            if count == 3 {
                break;
            } else {
                continue;
            }
        }
        print!("{} ", num); // This line is skipped due to continue
    }
    */
    
    let mut num = 0;
    let mut count = 0;
    let third_highest: i32 = loop {
        num += 1;
        if num % 5 == 0 && num % 3 == 0 {
            println!("\nThe number divisible by both 3 and 5 is: {}\n", num);
            count += 1;

            if count == 3 {
                break num;
            } else {
                continue;
            }
        }
        print!("{} ", num);
    };

    println!("The third highest number divisible by both 3 and 5 is {}", third_highest);
}
