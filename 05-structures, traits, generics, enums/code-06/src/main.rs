// -------------------------------------------
//          Option Enum
//            - Basic Syntax and Usage
// -------------------------------------------

/* General Syntax

enum Option<T> {
    None,
    Some(T),
}
*/

// Example 1
/*
fn main() {
    let mut health_condition: Option<String> = None;
    health_condition = Some(String::from("Hypertension"));
    
    match health_condition {
        Some(condition) => println!("You have the health condition: {}", condition),
        None => println!("You have no health conditions"),
    }
}
*/

// Example 2. Options with Different Types

/*
struct Individual {
    name: String,
    age: i32,
}

fn main() {
    let str_option: Option<&str> = Some("Sample Text");
    
    println!("\n The value of str_option is {:?}\n The actual value is {:?} \n\n", str_option, str_option.unwrap());

    let float_option: Option<f64> = Some(12.34);

    let mut sum = 20.0;
    sum += float_option.unwrap();
    println!("The total value is {}", sum);

    let vec_option: Option<Vec<i32>> = Some(vec![1, 2, 3, 4]);

    let person = Individual {
        name: String::from("Alex"),
        age: 28,
    };

    let person_option: Option<Individual> = Some(person);
}
*/

fn main() {
    let value = Some(4);

    if calculate_square(value) != None {
        println!("\n\n The result of the square is {:?} \n\n", calculate_square(Some(4)).unwrap());
    } else {
        println!("\n\n No value present \n\n");
    }
    calculate_square(None);
}

fn calculate_square(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(num) => Some(num * num),  // Wrap result in Some to match return type
        None => None,
    }
}
