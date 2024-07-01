fn main() {

    // -------------------------------------------------
    //           Vectors in Rust
    //           - Creating Vectors
    //           - Accessing Elements
    //           - Displaying Elements
    //           - Modifying Elements
    //           - Initializing with Repeated Values
    //           - Working with Strings and Characters in Vectors
    //           - Slicing Vectors
    //           - Common Vector Operations
    //           - Handling Out-of-Bounds Access
    // -------------------------------------------------

    // Creating a vector of integers with initial values.
    let mut num_vector: Vec<i32> = vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36];

    // Accessing and printing the first element of the vector.
    println!("First element: {}", num_vector[0]);

    // Displaying the entire vector.
    println!("Full vector: {:?}", num_vector);

    // Modifying the fifth element (index 4) of the vector.
    num_vector[4] = 50; 
    println!("Updated vector: {:?}", num_vector);

    // Initializing a vector with ten elements, all set to the value 1.
    let repeated_values_vector: Vec<i32> = vec![1; 8];

    // Creating a vector of string slices.
    let mut fruit_vector: Vec<&str> = vec!["apple", "banana", "cherry"];
    // Initializing a vector with six "unknown" string slices.
    let unknown_fruit_vector: Vec<&str> = vec!["unknown"; 5]; 
    // Changing the second element (index 1) of the string vector.
    fruit_vector[1] = "orange"; 

    // Creating a vector of characters.
    let char_vector: Vec<char> = vec!['r', 'u', 's', 't', 'c'];

    // Slicing the vector to get a subset of elements (from index 2 to 4).
    let sub_vector: &[i32] = &num_vector[2..5]; 
    println!("Subset of the vector: {:?}", sub_vector);

    // Printing the number of elements in the vector.
    println!("Number of elements in the vector: {}", num_vector.len());

    // Safely checking if an element exists at the specified index (100 in this case).
    let out_of_bounds_access = num_vector.get(100); 
    println!("Attempt to access out-of-bounds element: {:?}", out_of_bounds_access);

    // Adding new elements to the end of the vector.
    num_vector.push(45);
    num_vector.push(60);
    println!("Vector after adding elements: {:?}", num_vector);

    // Removing the element at the sixth position (index 5) of the vector.
    num_vector.remove(3); 
    println!("Vector after removing the element at index 3: {:?}", num_vector);

    // Checking if the value 10 is present in the vector.
    println!("Does the value 10 exist in the vector? {}", num_vector.contains(&10));
}
