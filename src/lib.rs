use ndarray::Array2;
use ort::Session;
use std::{path::Path, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Debug, Error)]
pub enum Error {
    #[error("onnxruntime: {0}")]
    OrtFailure(#[from] ort::Error),
}

#[derive(Debug)]
pub struct BelievexModel {
    session: Session,
}

impl BelievexModel {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self {
            session: Session::builder()?.commit_from_file(path)?,
        })
    }

    pub fn infer(&self, lf: f32, hf: f32) -> Result<f32, Error> {
        let mut input_tensor: Array2<f32> = Array2::zeros((1, 2));
        input_tensor[[0, 0]] = lf;
        input_tensor[[0, 1]] = hf;
        // dbg!(&self.session.inputs);
        let model_output = self
            .session
            .run(ort::inputs!["float_input" => input_tensor.view()]?)?;
        let output: Vec<f32> = model_output["variable"]
            .try_extract_tensor::<f32>()?
            .iter()
            .copied()
            .collect();

        Ok(*output.first().unwrap())
    }
}

pub type SharedBelievexModel = Arc<Mutex<BelievexModel>>;

#[derive(Debug, Clone)]
pub struct AsyncBelievexModelManager {
    pub mmc: SharedBelievexModel,
    pub mma: SharedBelievexModel,
    pub mfc: SharedBelievexModel,
    pub mfa: SharedBelievexModel,
}

impl AsyncBelievexModelManager {
    pub fn load_models<P: AsRef<Path>>(boys: P, girls: P, women: P, men: P) -> Result<Self, Error> {
        Ok(Self {
            mmc: Arc::new(Mutex::new(BelievexModel::load(boys)?)),
            mma: Arc::new(Mutex::new(BelievexModel::load(men)?)),
            mfc: Arc::new(Mutex::new(BelievexModel::load(girls)?)),
            mfa: Arc::new(Mutex::new(BelievexModel::load(women)?)),
        })
    }

    async fn infer_with_model(
        &self,
        model: &SharedBelievexModel,
        lf: f32,
        hf: f32,
    ) -> Result<f32, Error> {
        let lock = model.lock().await;
        lock.infer(lf, hf)
    }

    pub async fn infer(
        &self,
        lf: f32,
        hf: f32,
        is_male: bool,
        is_child: bool,
    ) -> Result<f32, Error> {
        match (is_male, is_child) {
            (true, true) => self.infer_with_model(&self.mmc, lf, hf).await,
            (true, false) => self.infer_with_model(&self.mma, lf, hf).await,
            (false, true) => self.infer_with_model(&self.mfc, lf, hf).await,
            (false, false) => self.infer_with_model(&self.mfa, lf, hf).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        AsyncBelievexModelManager::load_models(
            "models/knn_boys.onnx",
            "models/knn_girls.onnx",
            "models/knn_women.onnx",
            "models/knn_men.onnx",
        )
        .unwrap();
    }
}
