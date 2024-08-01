// --------------------------------------------------
//                  Lifetimes
//           - Invalid References
//           - Inferred Lifetimes
// --------------------------------------------------

//    use std::vec;

/*
fn main() {
    let ref_to_int: &i32;
    {
        let local_int = 42;
        ref_to_int = &local_int;
    }
    println!("The referenced value is {}", ref_to_int);
}
*/

/*
fn main() {
    let number = 20;
    let result = my_function(number);
    println!("{}", result);
}

fn my_function(num: i32) -> &i32 {
    &num
}
*/

/*
fn main() {
    let num1 = 15;
    let num2 = 30;
    let max_num = max_value(&num1, &num2);
}

fn max_value(x: &i32, y: &i32) -> &i32 {
    if x > y {
        x
    } else {
        y
    }
}
*/

fn main() {
    let greeting = "Hi";

    let message;
    {
        let word = String::from("Rust");
        message = combine_strings(greeting, word.as_str());
    }
    println!("\n\n{} \n\n", message);
}

fn combine_strings(str1: &str, str2: &str) -> &str {
    str1
}
