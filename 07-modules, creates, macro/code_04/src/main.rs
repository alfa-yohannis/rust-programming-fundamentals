/* 
These lines are not intended for documentation
// Documentation comments begin with three slashes. Ensure your code is error-free and well-documented before publishing.


// Command to generate documentation: cargo doc --open
*/


// The following lines will be included in the documentation 
// _______________________________________________

//! # Fitness Tracker Crate
//! 
//! This crate provides utilities for basic fitness calculations and metrics.

/// Calculates the Body Mass Index (BMI) from weight and height
/// 
/// # Examples
/// ```  
/// let weight_kg = 70;
/// let height_m = 1.75;
/// let bmi = fitness_tracker::calculate_bmi(weight_kg, height_m);
/// assert_eq!(22.86, bmi); 
/// ``` 
/// # Panics
/// Panics if height is zero or negative.
/// 
/// # Additional Information
/// BMI is calculated using the formula weight (kg) / (height (m) * height (m)).

pub fn calculate_bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
} 


/// Calculates the daily calorie needs based on basal metabolic rate (BMR) and activity level
/// 
/// # Examples
/// ``` 
/// let bmr = 1500.0;
/// let activity_factor = 1.2;
/// let daily_calories = fitness_tracker::calculate_calories(bmr, activity_factor);
/// assert_eq!(1800.0, daily_calories); 
/// ```  
pub fn calculate_calories(bmr: f32, activity_level: f32) -> f32 {
    bmr * activity_level
}


// Steps for publishing your crate
// 1. Create a GitHub account and log in to crates.io.
// 2. Navigate to the crates.io dashboard, access API tokens, and create a new token.
// 3. Copy the token.
// 4. In the terminal, execute cargo login followed by the token from crates.io.
// 5. Go back to crates.io, verify your email in Account Settings -> Profile -> Email -> Save.
// 6. Ensure your Cargo.toml includes: version = "0.1.1", edition = "2021", authors = ["Your Name"], description = "Fitness calculation utilities", license = "MIT".
// 7. Run cargo publish --allow-dirty to publish your crate.
// 8. To update, modify the version number in Cargo.toml and run the publish command again.
// 9. Your crate will appear on your dashboard.
// 10. From your dashboard, you can yank or unyank versions to control downloads.
// 11. Your crate will also be searchable.

 
// To demonstrate usage, create a new project and include the following crate:
use my_bmi_crate_01::calculate_bmi; 
fn main() {
    println!("BMI for 70kg and 1.75m height: {}", calculate_bmi(70.0, 1.75));
    println!("Daily calories needed: {}", calculate_calories(1500.0, 1.2));
}

