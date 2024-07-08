use believex_backend::{AsyncBelievexModelManager, Error};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AppState {
    pub model_manager: AsyncBelievexModelManager,
}

impl AppState {
    pub fn new<P: AsRef<Path>>(boys: P, girls: P, women: P, men: P) -> Result<Self, Error> {
        Ok(Self {
            model_manager: AsyncBelievexModelManager::load_models(boys, girls, women, men)?,
        })
    }
}
