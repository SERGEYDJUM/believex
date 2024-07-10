use believex_backend::AsyncBelievexModelManager;
pub use believex_backend::{Error, MMConfig};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub model_manager: Arc<AsyncBelievexModelManager>,
}

impl AppState {
    pub fn new(model_config: MMConfig<&str>) -> Result<Self, Error> {
        Ok(Self {
            model_manager: Arc::new(AsyncBelievexModelManager::load_models(model_config)?),
        })
    }
}
