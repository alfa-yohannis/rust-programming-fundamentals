// -------------------------------------------
//          Iterators
//            - Basics
//            - Useful Functions
//            - Common Statistics
// -------------------------------------------

// Basic Usage of Iterators
/*
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let mut iterator = numbers.iter();

    println!("The iterator: {:?}", iterator);
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
}
*/

// Using Useful Functions with Iterators
fn main() {
    let values: Vec<u32> = vec![0, 1, 2, 4, 5, 6, 9, 8, 7];

    let any_check = values.iter().any(|&x| x > 0);
    println!("The result of the any function is {}", any_check);

    let all_check = values.iter().all(|&x| x > 0);
    println!("The result of the all function is {}", all_check);

    let find_check = values.iter().find(|&&x| x > 0);
    println!("The result of the find function is {}", find_check.unwrap());

    let position_check = values.iter().position(|&x| x > 4);
    println!(
        "The result of the position function is {}",
        position_check.unwrap()
    );

    let rposition_check = values.iter().rposition(|&x| x > 4);
    println!(
        "The result of the rposition function is {}",
        rposition_check.unwrap()
    );

    let max_check = values.iter().max();
    println!("The result of the max function is {}", max_check.unwrap());

    let min_check = values.iter().min();
    println!("The result of the min function is {}", min_check.unwrap());

    let sum_check: u32 = values.iter().sum();
    let product_check: u32 = values.iter().product();
    println!("Sum and product: {} {}", sum_check, product_check);

    let mut reversed_iter = values.iter().rev();
    println!(
        "The result of applying the rev function {:?}",
        reversed_iter.collect::<Vec<_>>()
    );
    println!("Original vector: {:?}", values);
}
