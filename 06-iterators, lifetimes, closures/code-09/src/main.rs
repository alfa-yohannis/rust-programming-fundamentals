// Example: Set Union and Intersection with Different Values

fn main() {
    // Initialize two vectors with different values
    let mut set1: Vec<u32> = vec![2, 7, 8, 11, 14];
    let mut set2: Vec<u32> = vec![2, 6, 8, 10, 13, 18, 22];

    // Compute the intersection of the two sets
    let common_elements = find_intersection(&set1, &set2);
    println!("\n\n The intersection of the two sets is {:?}", common_elements);

    // Compute the union of the two sets, excluding the common elements
    let union_set = compute_union(&mut set1, &mut set2, &common_elements);
    println!("\n\n The union of the sets is {:?}", union_set);

    /* Alternative method for intersection
    let set1_copy = set1.clone();

    // Alternative way to find common elements
    let common_elements: Vec<u32> = set1.into_iter()
        .filter(|&x| set2.iter().any(|&y| y == x))
        .collect();
    println!("The common values are {:?}", common_elements);
    // This approach will consume set1

    println!("The uncommon values are {:?}", uncommon_elements);
    */
}

// Function to find the intersection of two sets
fn find_intersection(set1: &Vec<u32>, set2: &Vec<u32>) -> Vec<u32> {
    let mut common_elements: Vec<u32> = Vec::new();

    for item in set1 {
        if set2.iter().any(|&x| x == *item) {
            common_elements.push(*item);
        }
    }
    common_elements
}

// Function to compute the union of two sets, excluding common elements
fn compute_union<'a>(set1: &'a mut Vec<u32>, set2: &'a mut Vec<u32>, common_elements: &'a Vec<u32>) -> Vec<&'a u32> {
    for item in common_elements {
        if let Some(pos1) = set1.iter().position(|&x| x == *item) {
            set1.remove(pos1);
        }
        if let Some(pos2) = set2.iter().position(|&x| x == *item) {
            set2.remove(pos2);
        }
    }
    let union_set = set1.iter()
        .chain(set2.iter())
        .chain(common_elements.iter())
        .collect::<Vec<_>>();
    union_set
}
