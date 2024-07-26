// Define a structure for Employee
struct Employee {
    nationality: String,
    full_name: String,
    years: u8,
    gender: char,
    salary: i32,
}

struct Learner {
    student_name: String,
    years: u8,
    gender: char,
    nation: String,
}

trait PersonalDetails {
    fn get_details(&self) -> (&str, u8, char);

    // Additional methods for the trait
    fn get_nation(&self) -> &str;
}

impl PersonalDetails for Employee {
    fn get_details(&self) -> (&str, u8, char) {
        (&self.full_name, self.years, self.gender)
    }

    fn get_nation(&self) -> &str {
        &self.nationality
    }
}

impl PersonalDetails for Learner {
    fn get_details(&self) -> (&str, u8, char) {
        (&self.student_name, self.years, self.gender)
    }

    fn get_nation(&self) -> &str {
        &self.nation
    }
}

// fn main() {
//     // Create an Employee instance with specific values
//     let emp1 = Employee {
//         full_name: String::from("John Doe"),
//         nationality: String::from("Canada"),
//         years: 45,
//         gender: 'M',
//         salary: 55_000,
//     };

//     // Create a Learner instance
//     let learner1 = Learner {
//         student_name: String::from("Jane Doe"),
//         years: 20,
//         gender: 'F',
//         nation: String::from("Australia"),
//     };

//     println!("Employee details: {:?}", emp1.get_details());
//     println!("Learner details: {:?}", learner1.get_details());
// }

// Example 2: Traits with Default Implementation
struct Sphere {
    radius: f32,
}

struct Square {
    side_length: f32,
}

trait ShapeMetrics {
    fn compute_area(&self);

    // Default implementation for perimeter
    fn compute_perimeter(&self) {
        println!("Perimeter calculation not provided for this shape.");
    }
}

impl ShapeMetrics for Sphere {
    fn compute_area(&self) {
        let area_of_sphere = 4.0 * 3.14 * (self.radius * self.radius);
        println!("The surface area of the sphere is {}", area_of_sphere);
    }

    fn compute_perimeter(&self) {
        let circumference = 2.0 * 3.14 * self.radius;
        println!("The circumference of the sphere is {}", circumference);
    }
}

impl ShapeMetrics for Square {
    fn compute_area(&self) {
        let area_of_square = self.side_length * self.side_length;
        println!("The area of the square is {}", area_of_square);
    }

    fn compute_perimeter(&self) {
        let perimeter_of_square = 4.0 * self.side_length;
        println!("The perimeter of the square is {}", perimeter_of_square);
    }
}

fn main() {
    let sphere = Sphere { radius: 4.0 };
    let square = Square { side_length: 6.0 };

    sphere.compute_area();
    sphere.compute_perimeter();

    square.compute_area();
    square.compute_perimeter();
}
