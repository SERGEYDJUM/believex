use ndarray::Array2;
use ort::Session;
use std::path::Path;
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Debug, Error)]
pub enum Error {
    #[error("onnxruntime: {0}")]
    OrtFailure(#[from] ort::Error),
}

#[derive(Debug)]
pub struct BelievexModel {
    lf_session: Session,
    hf_session: Session,
}

impl BelievexModel {
    pub fn load<P: AsRef<Path>>(lf_path: P, hf_path: P) -> Result<Self, Error> {
        Ok(Self {
            lf_session: Session::builder()?.commit_from_file(lf_path)?,
            hf_session: Session::builder()?.commit_from_file(hf_path)?,
        })
    }

    pub fn infer(&self, lf: f32, hf: f32) -> Result<(f32, f32), Error> {
        let mut input_ndarr: Array2<f32> = Array2::zeros((1, 2));
        input_ndarr[[0, 0]] = lf;
        input_ndarr[[0, 1]] = hf;

        let lf_model_output = self
            .lf_session
            .run(ort::inputs!["float_input" => input_ndarr.view()]?)?;

        let hf_model_output = self
            .hf_session
            .run(ort::inputs!["float_input" => input_ndarr.view()]?)?;

        let lf_output: Vec<f32> = lf_model_output["variable"]
            .try_extract_tensor::<f32>()?
            .iter()
            .copied()
            .collect();

        let hf_output: Vec<f32> = hf_model_output["variable"]
            .try_extract_tensor::<f32>()?
            .iter()
            .copied()
            .collect();

        Ok((lf_output[0], hf_output[0]))
    }
}

pub type LockedBelievexModel = Mutex<BelievexModel>;

#[derive(Debug)]
pub struct AsyncBelievexModelManager {
    pub male_2h: LockedBelievexModel,
    pub male_5d: LockedBelievexModel,
    pub female_2h: LockedBelievexModel,
    pub female_5d: LockedBelievexModel,
    pub child_male_2h: LockedBelievexModel,
    pub child_male_5d: LockedBelievexModel,
    pub child_female_2h: LockedBelievexModel,
    pub child_female_5d: LockedBelievexModel,
}

#[derive(Debug, Clone, Copy)]
pub struct MMConfig<P: AsRef<Path>> {
    pub mlf2h: P,
    pub mlf5d: P,
    pub mhf2h: P,
    pub mhf5d: P,
    pub flf2h: P,
    pub flf5d: P,
    pub fhf2h: P,
    pub fhf5d: P,
    pub cmlf2h: P,
    pub cmlf5d: P,
    pub cmhf2h: P,
    pub cmhf5d: P,
    pub cflf2h: P,
    pub cflf5d: P,
    pub cfhf2h: P,
    pub cfhf5d: P,
}

impl AsyncBelievexModelManager {
    pub fn load_models<P: AsRef<Path>>(config: MMConfig<P>) -> Result<Self, Error> {
        Ok(Self {
            male_2h: Mutex::new(BelievexModel::load(config.mlf2h, config.mhf2h)?),
            male_5d: Mutex::new(BelievexModel::load(config.mlf5d, config.mhf5d)?),
            female_2h: Mutex::new(BelievexModel::load(config.flf2h, config.fhf2h)?),
            female_5d: Mutex::new(BelievexModel::load(config.flf5d, config.fhf5d)?),
            child_male_2h: Mutex::new(BelievexModel::load(config.cmlf2h, config.cmhf2h)?),
            child_male_5d: Mutex::new(BelievexModel::load(config.cmlf5d, config.cmhf5d)?),
            child_female_2h: Mutex::new(BelievexModel::load(config.cflf2h, config.cfhf2h)?),
            child_female_5d: Mutex::new(BelievexModel::load(config.cflf5d, config.cfhf5d)?),
        })
    }

    pub async fn infer(
        &self,
        lf: f32,
        hf: f32,
        is_child: bool,
        is_male: bool,
        is_late: bool,
    ) -> Result<(f32, f32), Error> {
        match (is_child, is_male, is_late) {
            (false, true, false) => self.male_2h.lock().await.infer(lf, hf),
            (false, true, true) => self.male_5d.lock().await.infer(lf, hf),
            (false, false, false) => self.female_2h.lock().await.infer(lf, hf),
            (false, false, true) => self.female_5d.lock().await.infer(lf, hf),
            (true, true, false) => self.child_male_2h.lock().await.infer(lf, hf),
            (true, true, true) => self.child_male_5d.lock().await.infer(lf, hf),
            (true, false, false) => self.child_female_2h.lock().await.infer(lf, hf),
            (true, false, true) => self.child_female_5d.lock().await.infer(lf, hf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mm_config = MMConfig {
            mlf2h: "models/knn_male_adults_lf_2h.onnx",
            mlf5d: "models/knn_male_adults_lf_5d.onnx",
            mhf2h: "models/knn_male_adults_hf_5d.onnx",
            mhf5d: "models/knn_male_adults_hf_5d.onnx",
            flf2h: "models/knn_female_adults_lf_2h.onnx",
            flf5d: "models/knn_female_adults_lf_5d.onnx",
            fhf2h: "models/knn_female_adults_hf_2h.onnx",
            fhf5d: "models/knn_female_adults_hf_5d.onnx",
            cmlf2h: "models/knn_male_child_lf_2h.onnx",
            cmlf5d: "models/knn_male_child_lf_5d.onnx",
            cmhf2h: "models/knn_male_child_hf_5d.onnx",
            cmhf5d: "models/knn_male_child_hf_5d.onnx",
            cflf2h: "models/knn_female_child_lf_2h.onnx",
            cflf5d: "models/knn_female_child_lf_5d.onnx",
            cfhf2h: "models/knn_female_child_hf_2h.onnx",
            cfhf5d: "models/knn_female_child_hf_5d.onnx",
        };

        AsyncBelievexModelManager::load_models(mm_config).unwrap();
    }
}
