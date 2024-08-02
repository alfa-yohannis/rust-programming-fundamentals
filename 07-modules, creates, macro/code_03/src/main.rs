// -------------------------------------------
//          Third-Party Libraries
//              - Include in Cargo.toml under the dependencies array_tool = "1.0.3"
// -------------------------------------------

use array_tool::vec::*;

fn main() {
    let list_a = vec![4, 4, 6, 8, 9, 10];
    let list_b = vec![4, 5, 6];
    
    let overlap = list_a.intersect(list_b.clone());
    println!("The overlap = {:?}", overlap);

    let combined_set = list_a.union(list_b.clone());
    println!("The combined set = {:?}", combined_set);
    
    println!("List B repeated three times = {:?}", list_b.times(3));
}
