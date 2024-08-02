fn display_message() {
    println!("This is a message from the primary_module crate");
}


/*
primary_module
        - arithmetic
            - core_operations
        area_of_rectangle()
        display_message()
 */

pub mod arithmetic {
    pub mod core_operations {
        pub fn multiply(x: &i32, y: &i32) -> i32 {
            let product = x * y;
            show_result(&product);
            product
        }
        fn show_result(product: &i32) {
            println!("The computed result is {}", product);
            crate::module_a::display_message();
        }
    }
}

// Note: Removing the pub keyword will cause the program to fail to compile
pub fn area_of_rectangle(length: &i32, width: &i32) -> i32 {
    // Using absolute path
    use arithmetic::core_operations::multiply;
    multiply(length, width)  // Parent modules cannot access child modules, but child modules can access parent modules by default
}

