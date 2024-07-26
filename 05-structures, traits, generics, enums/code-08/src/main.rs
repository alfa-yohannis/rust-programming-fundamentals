// -------------------------------------------
//          Hash Maps
//            - Creating hash maps
//            - Adding entries
//            - Retrieving entries
//            - Checking existence of keys
//            - Iterating over entries
//            - Modifying existing entries
// -------------------------------------------

use std::collections::HashMap;

// Example 1: Basic Operations
/*
fn main() {
    let mut people_ages: HashMap<&str, i32> = HashMap::new();
    people_ages.insert("Alice", 30);
    people_ages.insert("Bob", 25);
    people_ages.insert("Charlie", 35);

    println!("Alice's age is {:?}", people_ages.get("Alice").unwrap());

    if people_ages.contains_key("Alice") {
        println!("The key 'Alice' is present.");
    } else {
        println!("The key 'Alice' is not found.");
    }

    match people_ages.get("Alice") {
        Some(age) => println!("Alice's age: {}", age),
        None => println!("No entry for Alice."),
    }

    for (name, age) in &people_ages {
        println!("{} is {} years old", name, age);
    }
}
*/

// Example 2: Using `entry` API
/*
fn main() {
    let mut favorite_fruits: HashMap<&str, &str> = HashMap::new();
    
    favorite_fruits.entry("Alice").or_insert("banana");
    favorite_fruits.entry("Alice").or_insert("cherry");

    println!("Alice's favorite fruits: {:?}", favorite_fruits);
}
*/

// Example 3: Counting Frequencies
fn main() {
    let numbers: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let mut counts: HashMap<i32, u32> = HashMap::new();

    for number in &numbers {
        let counter: &mut u32 = counts.entry(*number).or_insert(0);
        *counter += 1;
    }

    println!("Number frequencies: {:?}", counts);
}
