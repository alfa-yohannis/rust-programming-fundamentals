// -------------------------------------------
//          Enums
//            - Basic Syntax
//            - Enums with Associated Data
//            - Enums for Mixed Data Types in Collections
// -------------------------------------------

enum Transportation {
    Bike, 
    Bus,
    Plane,
}

impl Transportation {
    fn reimbursement(&self, distance: i32) -> f32 {
        let rate = match self {
            Transportation::Bike => distance as f32 * 12.0 * 1.5,  
            Transportation::Bus => distance as f32 * 16.0 * 1.5,
            Transportation::Plane => distance as f32 * 25.0 * 1.5,
        }; 
        rate
    }
}

// fn main() {
//     let traveler1 = Transportation::Bike;

//     println!("The enum value for traveler1 is {}", traveler1 as i32);
//     let traveler2 = Transportation::Plane; 
//     let traveler3 = Transportation::Bus;

//     println!("Traveler 1 is entitled to a reimbursement of {}", traveler1.reimbursement(70)); 
//     println!("Traveler 2 is entitled to a reimbursement of {}", traveler2.reimbursement(150)); 
//     println!("Traveler 3 is entitled to a reimbursement of {}", traveler3.reimbursement(70)); 
// }

// Example 2: Enums with Attached Data
enum Transport {
    Bicycle(i32), 
    Coach(i32),
    Aircraft(i32),
}

impl Transport {
    fn reimbursement(&self) -> f32 {
        let rate = match self {
            Transport::Bicycle(distance) => *distance as f32 * 12.0 * 1.5,  
            Transport::Coach(distance) => *distance as f32 * 16.0 * 1.5,
            Transport::Aircraft(distance) => *distance as f32 * 25.0 * 1.5,
        }; 
        rate
    }
}

// fn main() {
//     let commuter1 = Transport::Bicycle(70); 
//     let commuter2 = Transport::Aircraft(150); 
//     let commuter3 = Transport::Coach(70); 

//     println!("Commuter 1 is entitled to a reimbursement of {}", commuter1.reimbursement()); 
//     println!("Commuter 2 is entitled to a reimbursement of {}", commuter2.reimbursement()); 
//     println!("Commuter 3 is entitled to a reimbursement of {}", commuter3.reimbursement()); 
// }

// Example 3: Enums for Mixed Data Types
#[derive(Debug)]
enum Variant {
    Integer(i32), 
    Decimal(f32),
}

fn main() {
    let values = vec![Variant::Integer(42), Variant::Decimal(22.5)]; 
    println!("\n\nThe integer value is {:?}, the float value is {:?}", values[0], values[1]); 

    for item in values.iter() { 
        match item {
            Variant::Integer(num) => println!("The value is an integer: {}", num), 
            Variant::Decimal(num) => println!("The value is a float: {}", num),  
        }
    }
}
