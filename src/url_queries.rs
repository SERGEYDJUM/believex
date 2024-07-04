use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestQuery {
    pub testval: usize,
}
