fn main() {

    // -------------------------------------------
    // 		Reference Rules  
    //          - At most one mutable reference in a scope
    //          - Multiple immutable references allowed
    //          - Mutable and immutable references cannot coexist
    //          - Scope of a reference
    //          - Data must remain unchanged when immutable references are active
    // -------------------------------------------

    /*

    let mut vector_data = vec![7, 8, 9]; 
    let ref1 = &mut vector_data; 
    let ref2 = &mut vector_data; 
    
    println!("The first mutable reference is {:?} and the second mutable reference is {:?}", ref1, ref2);

    let mut vector_data = vec![7, 8, 9]; 
    let ref1 = &vector_data; 
    let ref2 = &vector_data; 
    println!("The first immutable reference is {:?} and the second immutable reference is {:?}", ref1, ref2);

    let mut vector_data = vec![7, 8, 9]; 
    let ref1 = &vector_data; 
    let ref2 = &vector_data; 
    let ref3 = &mut vector_data;  
    println!("Immutable references are {:?} and {:?} and the mutable reference is {:?}", ref1, ref2, ref3);

    let mut vector_data = vec![7, 8, 9]; 
    let ref1 = &vector_data; 
    let ref2 = &vector_data; 
    println!("Immutable references are {:?} and {:?}", ref1, ref2); 
    let ref3 = &mut vector_data;   
    */

    let mut vector_data = vec![7, 8, 9]; 
    vector_data.push(78);

    let ref1 = &vector_data; 
    let ref2 = &vector_data; 
    println!("Immutable references are {:?} and {:?}", ref1, ref2); 
    vector_data.push(98);
}
