// -------------------------------------------
//          Closures  
//            - A Brief Overview
//            - Borrowing by Immutable Reference
//            - Borrowing by Mutable Reference
//            - Moving a Value into a Closure
// -------------------------------------------

// Brief Overview of Closures
/*
fn main() {
    let increment_1 = |x: u32| -> u32 { x + 1 };
    let increment_2 = |x| { x + 1 };
    let increment_3 = |x| x + 1;
}
*/

// Borrowing by Immutable Reference
/*
fn main() {
    let mut numbers = vec![1, 2, 3];
    let display_numbers = || {
        // Accessing numbers by reference.
        println!("Numbers: {:?}", numbers);
    };

    println!("Numbers: {:?}", numbers);
    display_numbers();

    numbers[1] = 15;
}
*/

// Borrowing by Mutable Reference
/*
fn main() {
    let mut numbers = vec![4, 5, 6];
    let mut add_to_numbers = || {
        numbers.push(35);
    };

    // Displaying numbers is commented out.
    // numbers[1] = 15;
    add_to_numbers();

    // Modifying numbers is commented out.
    // numbers[2] = 15;
}
*/

// Moving a Value into a Closure
fn main() {
    let mut numbers_1 = vec![1, 2, 3];
    let handle_numbers = || {
        let numbers_2 = numbers_1;
    };

    handle_numbers();
    // Accessing numbers_1 after moving is not possible.
    // println!("Numbers 1 = {:?}", numbers_1);
    // println!("Numbers 2 = {:?}", numbers_2);
}
