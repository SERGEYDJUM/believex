use anyhow::Ok;
use ort::Session;
use std::path::Path;

#[derive(Debug)]
pub struct BelievexModel {
    session: Session,
}

impl BelievexModel {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            session: Session::builder()?.commit_from_file(path)?,
        })
    }

    pub fn infer(&self) -> anyhow::Result<String> {
        Ok(self.session.metadata()?.name()?)
    }
}