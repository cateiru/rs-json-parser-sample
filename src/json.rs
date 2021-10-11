use crate::body::Body;
use crate::file::File;
use crate::parser::start;
use std::{error::Error, path::Path};

pub fn parse(path: &Path) -> Result<(), Box<dyn Error>> {
    let file_op = File::new(path);

    if let Some(file_op) = file_op {
        start(&mut Body::new(&file_op.read_all()?));
    }

    Ok(())
}
