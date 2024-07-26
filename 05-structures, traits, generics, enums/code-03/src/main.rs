// -------------------------------------------
//          Traits
//            - Overview
//            - Default method implementation
//            - Functions within a trait
// -------------------------------------------

struct Metrics {
    values: Vec<i32>,
}

trait Statistical {
    fn average(&self) -> f32; 
    fn dispersion(&self) -> f32;  
}

impl Statistical for Metrics {
    fn average(&self) -> f32 {
        let mut total = 0; 
        for value in self.values.iter() {
            total += *value; 
        }
        //println!("{:?}",total); 
        total as f32 / self.values.len() as f32
    }

    fn dispersion(&self) -> f32 {
        let avg = self.average();  
        let mut squared_diff_sum: f32 = 0.0;
        for value in self.values.iter() {
            squared_diff_sum += (*value as f32 - avg) * (*value as f32 - avg);
        } 
        squared_diff_sum / self.values.len() as f32
    }
}

fn main() {
    let dataset = Metrics {
        values: vec![4, 7, 5, 9, 8, 6, 10],
    };
    println!("The average of the dataset is {}", dataset.average()); 
    println!("The variance of the dataset is {}", dataset.dispersion()); 
}
