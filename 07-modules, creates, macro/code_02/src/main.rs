mod module_a;
fn main() {
    let rectangle = Rectangle {
        height: 5,
        breadth: 10,
    };
    let rectangle_area = calculate_area(&rectangle.height, &rectangle.breadth); // remove module_x:: if included in the same script
}

struct Rectangle {
    height: i32,
    breadth: i32,
}

pub fn calculate_area(height: &i32, breadth: &i32) -> i32 {
    use module_a::arithmetic::core_operations::multiply;
    multiply(height, breadth)
}

// fn display_message() {
//     println!("This is a message from the primary_module crate");
// }


// /*
// primary_module
//         - arithmetic
//             - core_operations
//         area_of_rectangle()
//         display_message()
//  */

// pub mod arithmetic {
//     pub mod core_operations {
//         pub fn multiply(x: &i32, y: &i32) -> i32 {
//             let product = x * y;
//             show_result(&product);
//             product
//         }
//         fn show_result(product: &i32) {
//             println!("The computed result is {}", product);
//             crate::component_a::display_message();
//         }
//     }
// }

// // Note: Removing the pub keyword will cause the program to fail to compile
// pub fn area_of_rectangle(length: &i32, width: &i32) -> i32 {
//     // Using absolute path
//     use arithmetic::core_operations::multiply;
//     multiply(length, width)  // Parent modules cannot access child modules, but child modules can access parent modules by default
// }


// mod module_b;
// fn main() {
//     module_b::create_animals();
// }

// mod module_c;
// fn main()
// {
//     module_c::calculate_salary();
// }
