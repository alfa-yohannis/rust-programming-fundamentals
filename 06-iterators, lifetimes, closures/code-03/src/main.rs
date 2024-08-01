// -------------------------------------------
//          Closures  
//            - Basic Structure 
//            - Closure with Parameters
//            - Same Variable Name for Multiple Closures
//            - Ownership with Closures
//            - Inferring Inputs and Outputs
//            - Passing Closure as Function Argument
// -------------------------------------------

// Basic Closure
/*
fn main(){
    let a = 5;
    let calculate_square = || println!("\n\n Square of a is {} \n\n", a * a);
    calculate_square();
}
*/

// Closure with Parameters
/*
fn main(){
    let a = 5;
    let compute_square = |value: i32| println!("\n\n Square of {} is {} \n\n", value, value * value);
    compute_square(a);

    let b = 15;
    compute_square(b);
}
*/

// Multiple Closures with Same Variable Name
/*
fn main(){
    let a = 5;
    let compute_square = |value: i32| println!("\n\n Square is {}", value * value);
    let compute_cube = |value: i32| println!("\n\n Cube is {} \n\n", value * value * value);
    compute_cube(a);

    let b = 15;
    compute_cube(b);
}
*/

// Ownership and Closures
/*
fn main() {
    let display_user_info = |info: String, name: &str, age| println!("{}\n\t{}: {}", info, name, age);
    let info = String::from("User Information:");
    let (name, age) = (String::from("Alex"), 30);

    display_user_info(info, &name, age);
    println!("Variable after move: {}", name);
}
*/

// Inferring Inputs and Outputs
/*
fn main(){
    let compute_square = |value| value * value;

    let a = 5;
    compute_square(a);

    let b = 105.5;
    compute_square(b);
}
*/

// Passing Closure as Function Argument

fn divide<F: Fn(f32) -> bool>(numerator: f32, denominator: f32, is_valid: F) {
    if is_valid(denominator) {
        println!("\n\n Result is {} \n\n", numerator / denominator);
    } else {
        println!("\n\n Division by zero is not allowed \n\n");
    }
}

fn main() {
    let check_denominator = |denom: f32| denom != 0.0;
    divide(5.0, 10.0, check_denominator);
    divide(54.0, 0.0, check_denominator);
}
