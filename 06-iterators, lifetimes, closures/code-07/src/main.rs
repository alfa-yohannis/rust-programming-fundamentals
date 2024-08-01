// -------------------------------------------
//          Iterators   
//            - Basics
//            - Useful Functions
//            - Common Statistics
//            - Modifying and Collecting Values
// -------------------------------------------

fn main() {
    let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // Filtering values that are greater than or equal to 5 and collecting as references
    let filtered_refs = numbers.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
    println!("Filtered references: {:?}", filtered_refs);

    // Filtering values that are greater than or equal to 5 and collecting as owned values
    let cloned_numbers = numbers.clone();
    let filtered_values = cloned_numbers.into_iter().filter(|x| *x >= 5).collect::<Vec<u32>>();
    println!("Filtered values: {:?}", filtered_values);

    // Mapping values to their double
    let mapped_values = cloned_numbers.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("Mapped values: {:?}", mapped_values);

    // Mapping values to their double, then filtering values greater than 10
    let filtered_mapped_values = cloned_numbers.iter().map(|x| 2 * x).filter(|x| *x > 10).collect::<Vec<u32>>();
    println!("Filtered and mapped values: {:?}", filtered_mapped_values);
}
