use ndarray::Array2;
use ort::{Session, SessionInputValue};
use std::{borrow::Cow, path::Path};
use thiserror::Error;

pub type OrtInputTensor<'a> = Vec<(Cow<'a, str>, SessionInputValue<'a>)>;

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

    fn encode<'a>() -> Result<OrtInputTensor<'a>, Error> {
        let mut input_tensor: Array2<f32> = Array2::zeros((1, 2));
        input_tensor[[0, 0]] = 343.0;
        input_tensor[[0, 1]] = 422.0;
        Ok(ort::inputs!["float_input" => input_tensor.view()]?)
    }

    pub fn infer(&self) -> Result<f32, Error> {
        // dbg!(&self.session.inputs);
        let model_output = self.session.run(Self::encode()?)?;
        let output: Vec<f32> = model_output["variable"]
            .try_extract_tensor::<f32>()?
            .iter()
            .copied()
            .collect();

        Ok(*output.first().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let model = BelievexModel::load("models/knn_women.onnx").unwrap();
        model.session.metadata().unwrap();
    }
}
