use std::error::Error;
use std::path::Path;

pub trait Reader {
    type Output;

    fn evaluate(&self, path: &Path) -> Result<Self::Output, Box<dyn Error>>;
}
