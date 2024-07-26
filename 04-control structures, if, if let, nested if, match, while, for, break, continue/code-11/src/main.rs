fn main() {
    // Iterate over possible values of a, b, and c
    for a in 1..=1000 {
        for b in a + 1..1000 {
            for c in b + 1..1000 {
                // Check if the numbers form a Pythagorean triplet and their sum is 1000
                if a * a + b * b == c * c && a + b + c == 1000 {
                    println!("\n\nThe required Pythagorean triplet is ({}, {}, {})\n\n", a, b, c);
                    // Exit the program after finding the triplet
                    return;
                }
            }
        }
    }
}
