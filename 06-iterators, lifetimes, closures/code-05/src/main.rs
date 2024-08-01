// -------------------------------------------
//          Function Types   
//            - Basic Syntax and Usage
//            - Passing Function Types as Parameters
// -------------------------------------------

// Basic Syntax and Usage of Function Types
/*
fn maximum(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn minimum(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn main() {
    let mut chosen_function = maximum;
    println!("The minimum of the two values is {}", chosen_function(2, 3));
}
*/

// Passing Function Types as Parameters
/*
fn display_name(name: &str) {
    print!("The name is {}", name); 
}

fn show_details(func: fn(&str), person: &str, age: i32) {
    func(person); 
    println!(" and my age is {}", age);
}

fn main() {
    let (name, age) = (String::from("Nouman"), 40); 
    show_details(display_name, &name, age);
}
*/

// Using Function Types as Parameters
fn increment(x: i32) -> i32 {
    x + 1
}

fn apply_twice(func: fn(i32) -> i32, value: i32) -> i32 {
    func(value) + func(value)
}

fn main() {
    let result = apply_twice(increment, 5);
    println!("The result is: {}", result);
}
