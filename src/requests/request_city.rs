use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CityRequest {
    pub name: String,
    pub region: String
}
