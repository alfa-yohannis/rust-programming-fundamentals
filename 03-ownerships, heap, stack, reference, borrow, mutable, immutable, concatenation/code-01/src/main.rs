// -------------------------------------------
// 			Rust Ownership
// 			- Every value in Rust has an associated variable known as its owner.
// 			- Only one owner exists at any given time.
// 			- When the owner exits the scope, the value is deallocated.
// -------------------------------------------

fn main() {
    /*
    let mut a = 45.8; 
    let mut b = a;
    
    let str1 = String::from("xyz"); 
    let str2 = &str1; 
    println!("The value of str1 = {} and str2 = {}", str1, str2);   
    */ 

    /*
    let num_list1: Vec<i32> = vec![1, 2, 3, 4, 5];  
    // let num_list2 = num_list1;                   // Ownership transfer
    // println!("The first list is {:?} {:?}", num_list1, num_list2);

    let num_list2 = &num_list1;                   // Borrowing
    println!("The first list is {:?} {:?}", num_list1, num_list2);

    let num_list2 = num_list1.clone();
    println!("The first list is {:?} {:?}", num_list1, num_list2);
    */

    {
        let user_name = String::from("John Doe"); 
    } 
    println!("User name is {}", user_name); 
}
