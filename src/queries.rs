use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Sex {
    Female,
    Male,
}

#[derive(Debug, Deserialize)]
pub enum ObservationTime {
    After2h,
    After5d,
}

#[derive(Debug, Deserialize)]
pub struct InferenceQuery {
    pub sex: Sex,
    pub otime: ObservationTime,
    pub lf_b: f32,
    pub hf_b: f32,
}
