use crate::body::Body;
use crate::error::Error;
use crate::file::File;
use crate::parser::start;
use crate::value::Node;

use std::{error::Error as StdError, path::Path};

pub fn parse(path: &Path) -> Result<Box<dyn Node>, Box<dyn StdError>> {
    let file_op = File::new(path);

    if let Some(file_op) = file_op {
        let result = start(&mut Body::new(&file_op.read_all()?));
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(Box::new(Error::ParseError()))
        }
    } else {
        let path = path.to_str();

        if let Some(path) = path {
            Err(Box::new(Error::CannotOpenFileError(path.to_string())))
        } else {
            Err(Box::new(Error::CannotOpenFileError("".to_string())))
        }
    }
}
