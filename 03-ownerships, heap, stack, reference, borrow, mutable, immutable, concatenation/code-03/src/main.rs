// -------------------------------------------
// 			Rust Ownership
// 			- Every value in Rust has a variable known as its owner.
// 			- Only one owner can exist at any given time.
// 			- When the owner exits the scope, the value will be deallocated.
// -------------------------------------------

fn main() { 
    // -------------------------------------------
    // 			Ownership and Functions
    // -------------------------------------------
    
    /*
    let integer_value = 45;  
    let mut array_value = vec![7, 8, 9]; 
    
    process_stack(integer_value); 
    println!("The stack variable was copied, and the original value was {}", integer_value);
    
    process_heap(&mut array_value);  
    println!("The variable after the function call is {:?}", array_value);  
    */ 
    // -------------------------------------------
    // 			Quiz
    // -------------------------------------------

    let mut array_value = vec![7, 8, 9]; 
    let reference1 = array_value;    
    let reference2 = &reference1; 

    // -------------------------------------------
    // 			A Common Mistake
    // -------------------------------------------

    let mut array_value = vec![7, 8, 9]; 
    let mut reference1 = &array_value; 
    
    println!("Immutable references are {:?}", reference1); 

    // -------------------------------------------
    // 			When References are Useful
    // -------------------------------------------
    let long_text1 = String::from("This is the first lengthy string"); 
    let long_text2 = String::from("This is the second lengthy string"); 
    
    let combined_texts: Vec<&str> = vec![&long_text1, &long_text2];  
    println!("The combined strings are {:?}", combined_texts);
}
       
fn process_stack(mut value:i32)    
{   
    value = 78; 
    println!("The copied value of the variable has been updated to {}", value); 
}
       
fn process_heap(value:&mut Vec<i32>)    // without & in the first example
{
    value.push(60);    
    println!("The value of the vector inside the function is {:?}", value); 
}
