use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

use crate::ort_model::BelievexModel;

#[derive(Debug, Clone)]
pub struct AppState {
    pub ort_model: Arc<Mutex<BelievexModel>>,
}

impl AppState {
    pub fn new<P: AsRef<Path>>(model_path: P) -> anyhow::Result<Self> {
        Ok(Self {
            ort_model: Arc::new(Mutex::new(BelievexModel::load(model_path)?)),
        })
    }
}
