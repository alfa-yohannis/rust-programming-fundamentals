fn main() {

    // -------------------------------------------------
    // Functions in Rust
    // - Basic function (no parameters or return values)
    // - Function with inputs
    // - Function with variables as inputs
    // - Functions with inputs and outputs
    // - Functions with multiple outputs
    // - Code blocks
    // -------------------------------------------------

    call_basic_function(); // Invoking a basic function without parameters or return values

    display_employee_info("John", 40_000); // Using a function to display employee information

    let employee_name = "Alice";
    let employee_salary = 50_000;
    display_employee_info(employee_name, employee_salary); // Using variables as function inputs

    let multiplication_result = multiply_numbers(10, 15);
    println!("The result of multiplication is {}", multiplication_result);

    let (multiply_result, add_result, subtract_result) = perform_operations(10, 15);
    println!("Multiplication = {}, Addition = {}, Subtraction = {}", multiply_result, add_result, subtract_result);

    // Example of a code block
    let full_name = {
        let first_name = "Bob";
        let last_name = "Smith";
        format!("{} {}", first_name, last_name)
    };
    println!("Full name: {}", full_name);

    // Reading input from stdin and parsing it as a float
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read input.");
    let parsed_number: f64 = input_string.trim().parse().expect("Invalid input");
    println!("Parsed number: {:?}", parsed_number);
}

// Basic function (no parameters or return values)
fn call_basic_function() {
    println!("Executing a basic function");
}

// Function with inputs (string and integer)
fn display_employee_info(name: &str, salary: i32) {
    println!("Employee name: {} | Salary: {}", name, salary);
}

// Function with inputs and output (multiplication of two integers)
fn multiply_numbers(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

// Function with inputs and multiple outputs (multiplication, addition, subtraction)
fn perform_operations(num1: i32, num2: i32) -> (i32, i32, i32) {
    let multiplication = num1 * num2;
    let addition = num1 + num2;
    let subtraction = num1 - num2;
    (multiplication, addition, subtraction)
}
