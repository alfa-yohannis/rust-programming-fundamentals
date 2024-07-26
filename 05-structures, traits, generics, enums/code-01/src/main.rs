// Define a structure for Employee
struct Employee {
    nationality: String,
    full_name: String,
    years: i32,
    sex: char,
    income: i32,
}

impl Employee {
    // Initialize an Employee with default values
    fn create_default() -> Employee {
        Employee {
            nationality: String::from("Canada"),
            full_name: String::from("John Doe"),
            years: 30,
            sex: 'F',
            income: 50_000,
        }
    }

    // Calculate tax based on income
    fn calculate_tax(&self) -> f32 {
        (self.income as f32 / 4.0) * 0.4
    }
}

// fn main() {
//     // Create an Employee instance with specified values
//     let employee1 = Employee {
//         full_name: String::from("Alice Smith"),
//         nationality: String::from("UK"),
//         years: 35,
//         sex: 'F',
//         income: 60_000,
//     };

//     // Print employee details and calculated tax
//     println!("Employee details: {} {} {}", employee1.nationality, employee1.years, employee1.sex);
//     println!("Tax for {} is {}", employee1.full_name, employee1.calculate_tax());

//     // Create an Employee with default values
//     let employee2 = Employee::create_default();
//     println!("Default employee: Name {}, Nationality {}", employee2.full_name, employee2.nationality);

//     // Create and modify an Employee instance using fields from another instance
//     let employee3 = Employee {
//         years: 45,
//         full_name: String::from("Robert Brown"),
//         ..employee1
//     };
//     println!("Updated employee: Name = {}, Salary = {}", employee3.full_name, employee3.income);

//     // Modify and display another Employee instance
//     let mut employee4 = Employee::create_default();
//     println!("Initial name of employee 4: {}", employee4.full_name);
//     employee4.full_name = String::from("David Wilson");
//     println!("Updated name of employee 4: {}", employee4.full_name);
// }

// Define a tuple structure for Pair
struct Pair(i32, i32);

impl Pair {
    // Method to get the larger value
    fn larger(&self) -> i32 {
        if self.0 >= self.1 { self.0 } else { self.1 }
    }

    // Method to get the smaller value
    fn smaller(&self) -> i32 {
        if self.0 < self.1 { self.0 } else { self.1 }
    }
}

fn main() {
    // Create a Pair instance and display values
    let numbers = Pair(45, 23);
    println!("Numbers are: {} and {}", numbers.0, numbers.1);
    println!("Larger number: {}", numbers.larger());
    println!("Smaller number: {}", numbers.smaller());
}
