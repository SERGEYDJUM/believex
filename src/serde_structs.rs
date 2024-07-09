use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Sex {
    Female,
    Male,
}

// #[derive(Debug, Deserialize)]
// pub enum AgeGroup {
//     Child,
//     Adult,
// }

#[derive(Debug, Deserialize)]
pub struct InferenceQuery {
    pub sex: Sex,
    pub age: usize,
    pub lf: f32,
    pub hf: f32,
}
