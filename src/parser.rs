use crate::read::File;
use std::{error::Error, path::Path};

pub fn parse(path: &Path) -> Result<(), Box<dyn Error>> {
    let file_op = File::new(path);

    if let Some(file_op) = file_op {}

    Ok(())
}
