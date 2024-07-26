fn main() {
    // -------------------------------------------
    // Collecting Student Marks
    // - Using a while loop for repeated input
    // - Handling user decisions with conditionals
    // -------------------------------------------

    let mut continue_input = true;
    println!("Enter student marks as percentages:"); 

    let mut student_grades = Vec::new(); 
    while continue_input {
        println!("Please enter a student's marks:"); 

        let mut input_marks = String::new();                                       
        std::io::stdin()
            .read_line(&mut input_marks)
            .expect("Failed to read input.");

        let marks: i32 = input_marks.trim().parse().expect("Invalid input"); 
        student_grades.push(marks); 

        println!("Would you like to enter marks for another student? [Y/N]"); 

        let user_choice: char = {
            let mut choice_input = String::new();                                

            std::io::stdin()
                .read_line(&mut choice_input)
                .expect("Failed to read input.");

            choice_input.trim().parse().expect("Invalid input")
        };
        
        // Change to break instead of this conditional logic
        if user_choice == 'Y' {
            continue_input = true;    
        } else {
            continue_input = false;
        }
    }

    println!("The grades of the students are: {:?}", student_grades);
}
