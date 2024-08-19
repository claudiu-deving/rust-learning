use serde::{Serialize,Deserialize};
use crate::city::City;
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub cities: Vec<City>
}
