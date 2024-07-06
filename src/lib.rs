use ndarray::Array2;
use ort::{Session, SessionInputValue};
use thiserror::Error;
use std::{borrow::Cow, path::Path};

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

    pub fn infer(&self) -> Result<String, Error> {
        // dbg!(&self.session.inputs);
        let model_output = self.session.run(Self::encode()?)?;
        let output = model_output["variable"].try_extract_tensor::<f32>().into_iter().last().unwrap();
        Ok(output.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let model = BelievexModel::load("models/sample_model.onnx").unwrap();
        model.session.metadata().unwrap();
    }
}
