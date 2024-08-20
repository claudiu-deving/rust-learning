use serde::{Deserialize, Serialize};
use crate::apartment_data::ApartmentData;

#[derive(Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub apartments: Vec<ApartmentData>,
}
impl City {
    pub fn new(name: &String) -> Self {
        City {
            name:name.clone(),
            apartments: Vec::new(),
        }
    }
   pub fn calculate_averages(&self) -> (f64, f64, f64) {
       let data = &self.apartments[..];
        if data.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let count = data.len() as f64;
        let (sum1, sum2) = data.iter().fold((0u32, 0u32), |(acc1, acc2), item| {
            (acc1 + item.price as u32, acc2 + item.surface as u32)
        });

        let avg1 = sum1 as f64 / count;
        let avg2 = sum2 as f64 / count;

        // Avoid division by zero
        let ratio = if avg2 != 0.0 {
            avg1 / avg2
        } else {
            f64::INFINITY
        };

        (avg1, avg2, ratio)
    }
}
