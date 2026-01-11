use thiserror::Error;

pub mod global;
pub mod local;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub struct CacheId(String);
impl CacheId {
    pub fn new(id: &str) -> Self {
        Self(id.replace("/", "_"))
    }
}

impl ToString for CacheId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
