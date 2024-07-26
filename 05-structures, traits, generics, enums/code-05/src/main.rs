// -------------------------------------------
//          Generics
//            - Purpose (minimizing code repetition)
//            - Generics in functions
//            - Generics in structs
// -------------------------------------------

/*
fn square_int(n: i32) -> i32 {
    n * n
}

fn square_float(n: f32) -> f32 {
    n * n
}

fn main() {
    println!("The square of the integer is {}", square_int(5));
    println!("The square of the float is {}", square_float(5.0));
}
*/

/*
fn square<T>(n: T) -> T
where T: std::ops::Mul<Output = T> + Copy {
    n * n
}

fn main() {
    println!("The square of the integer is {}", square(5));
    println!("The square of the float is {}", square(5.5));
}
*/

/*
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let coord1 = Coordinate { x: 5, y: 5 };
    let coord2 = Coordinate { x: 1.0, y: 4.0 };
}
*/

/*
struct Coordinate<T> {
    x: T,
    y: T,
}

fn main() {
    let coord1 = Coordinate { x: 5, y: 5 };
    let coord2 = Coordinate { x: 1.0, y: 4.0 };
    // let coord3 = Coordinate { x: 5, y: 5.0 }; // Error: mismatched types
}
*/


struct Coordinate<T, U> {
    x: T,
    y: U,
}

impl<T: std::fmt::Debug, U: std::fmt::Debug> Coordinate<T, U> {
    fn display(&self) {
        println!("The values are {:?} and {:?}", self.x, self.y);
    }
}

fn main() {
    let coord1 = Coordinate { x: 5, y: 5 };
    coord1.display();
    let coord2 = Coordinate { x: 1.0, y: 4.0 };
    let coord3 = Coordinate { x: 5, y: 5.0 };
    coord3.display();
}

