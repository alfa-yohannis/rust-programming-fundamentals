fn main() {

    // --------------------------------------
    //           Variables in Rust 
    // --------------------------------------

    // Declaring a mutable variable `a` and initializing it with 10.
    let mut a = 10;  
    println!("The initial value of variable a = {}", a); 

    // Changing the value of `a` to 50.
    a = 50; 
    println!("The updated value of variable a = {}", a); 

    // Declaring an immutable variable `b` as the product of 6 * 6.
    let b = 6 * 6; 

    // --------------------------------------
    //           Data Types   
    // --------------------------------------

    // --------------------------------------
    //           Scalar Data Types
    //           - Integers 
    //              - Signed: i8, i16, i32, i64, i128
    //              - Unsigned: u8, u16, u32, u64, u128
    // --------------------------------------

    // Printing the maximum values for i8 and u8 integer types.
    println!("The Maximum value for i8 = {}", std::i8::MAX);
    println!("The Maximum value for u8 = {}", std::u8::MAX);

    // --------------------------------------
    //           - Floats   
    //              - f32, f64
    // --------------------------------------

    // Declaring a floating-point variable `c` and initializing it with 2.75.
    let c = 2.75;
    println!("The sum of an integer and a float is {}", a as f64 + c);
    println!("The Maximum value for f32 = {}", std::f32::MAX);

    // --------------------------------------
    //           - Boolean     
    // --------------------------------------

    // Declaring a boolean variable `is_active` and initializing it with `true`.
    let is_active = true; 
    println!("The values of our variables are {:?}", (a, b, c, is_active));

    // Declaring a boolean variable `is_not_equal` to check if 10 is not equal to 20.
    let is_not_equal = 10 != 20; // Other comparison operators can be >, <, >=, <=.
    println!("Is 10 not equal to 20? {}", is_not_equal);

    // --------------------------------------
    //           - Characters   
    //              - Represent single letters, 
    //              - digits,
    //              - emojis, or 
    //              - Unicode scalar values
    // --------------------------------------

    // Declaring character variables and initializing them with different types of characters.
    let char1 = 'X';
    let char2 = '8';
    let char3 = '&';
    let char4 = '\u{03A9}'; // Unicode character for Omega (Ω)
    let char5 = '\''; 

    println!("The value of char1 is {}, char2 is {}, char3 is {}, char4 is {} and char5 is {}", char1, char2, char3, char4, char5);
}
