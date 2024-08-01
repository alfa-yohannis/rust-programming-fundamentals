// -------------------------------------------
//          Lifetimes  
//          - Dangling References 
//          - Unclear Lifetimes
//          - Generic Lifetime Parameters (GLP)
//          - GLP often needed with function outputs that are references
//          - Problems with GLP
//          - GLP with multiple variables
//          - GLP and structs
//          - Reference to the same variable
// -------------------------------------------

/*
fn main() {
    let greeting = "Hello";
    let message;
    {
        let world = String::from("World");
        message = concatenate(greeting, world.as_str());
    }
    println!("{}", message);
}

fn concatenate<'a, 'b>(str1: &'a str, str2: &'b str) -> &'a str {
    str1
}
*/

/*
fn main() {
    let num1 = 5;
    let num2 = 10;
    let result = max_value(&num1, &num2);
}

fn max_value(x: &i32, y: &i32) -> i32 {
    if x > y {
        *x
    } else {
        *y
    }
}
*/

/*
fn main() {
    let num1 = 5;
    let num2 = 10;
    let result = max_value(&num1, num2);
}

fn max_value<'a>(x: &'a i32, y: i32) -> &'a i32 {
    x
}
*/

/*
fn main() {
    let num1 = 5;
    let num2 = 10;
    let result = max_value(&num1, &num2);
}

fn max_value<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}
*/

/*
fn main() {
    let num1 = 5;
    {
        let num2 = 10;
        let result = max_value(&num1, &num2);
        println!("The larger value is {}", result);
    }
}

fn max_value<'a, 'b>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}
*/

/*
struct Individual<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let first_name = "John";
    let mut person = Individual {
        name: first_name,
        age: 40,
    };

    {
        let last_name = String::from("Doe");
        person.name = &last_name;
    }

    println!("\n\nThe name of the person is {} and their age is {}\n\n", person.name, person.age);
}
*/

fn main() {
    let numbers: Vec<i32> = vec![5, 8, 9, 8, 7, 5, 2];
    let returned_vec = process_vec(&numbers, &numbers);
}

fn process_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
}
