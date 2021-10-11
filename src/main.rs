use json_parser::File;
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let file_op = File::new(Path::new("path/to"));

    if let Some(file_op) = file_op {
        let f = file_op.read_all()?;
        print!("{}", f);
    }
    Ok(())
}
