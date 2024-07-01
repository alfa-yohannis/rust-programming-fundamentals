fn main() {

    // -------------------------------------------------
    //           Tuples
    //           - Declaring tuples
    //           - Destructuring tuples
    //           - Nested tuples
    //           - Empty tuples
    // -------------------------------------------------

    // Declaring a tuple to store item information.
    let item_info = ("Item Price", 75_000);
    println!("{} is {}", item_info.0, item_info.1); 
    println!("The entire tuple is {:?}", item_info);

    // Destructuring the tuple into individual variables.
    let (description, price) = item_info;
    println!("The separated values are: {} and {}", description, price); 

    // Accessing tuple elements using dot notation.
    let item_desc = item_info.0;
    let item_price = item_info.1; 
    println!("Accessed individually: {} and {}", item_desc, item_price);

    // Creating a nested tuple.
    let complex_tuple = (7, 9.5, (2, 4), "World");
    let inner_element = complex_tuple.2.0;
    println!("The inner element is {}", inner_element); 

    // Declaring an empty tuple.
    let empty_tuple = ();

    // -------------------------------------------------
    //           Arrays
    //           - Declaring arrays
    //           - Accessing elements
    //           - Printing elements
    //           - Updating elements 
    //           - Initializing with same values
    //           - String and char arrays
    //           - Array slices
    //           - Common functions 
    //           - Invalid access 
    // -------------------------------------------------

    // Declaring and initializing an array of integers.
    let mut num_array: [i32; 5] = [10, 20, 30, 40, 50];  

    // Accessing and printing a specific element.
    println!("First element: {}", num_array[0]);

    // Printing the entire array.
    println!("Full array: {:?}", num_array);

    // Updating an element in the array.
    num_array[3] = 100; 
    println!("Updated array: {:?}", num_array);

    // Initializing an array with the same element.
    let repeated_values_array = [3; 8];

    // Declaring string arrays.
    let mut fruits = ["apple", "banana", "cherry"];
    let repeated_fruit = ["unknown"; 4]; 
    fruits[1] = "orange"; 

    // Declaring a character array.
    let char_array = ['r', 'u', 's', 't', 'y']; 

    // Creating a slice of the array.
    let subset_of_numbers: &[i32] = &num_array[1..4];  // Slices the array from index 1 to 3.
    println!("Array slice: {:?}", subset_of_numbers); 

    // Printing the number of elements in the array.
    println!("Number of elements in the array: {}", num_array.len());

    // Printing the memory size of the array.
    println!("Memory size of the array: {} bytes", std::mem::size_of_val(&num_array)); 

    // Attempting to access an invalid index (commented out to prevent panic).
    // num_array[10] = 5; // This will cause a runtime error if uncommented.

    // Checking for the existence of an element at a specific index.
    let element_at_index = num_array.get(10); 
    println!("Attempt to access out-of-bounds element: {:?}", element_at_index);
}
