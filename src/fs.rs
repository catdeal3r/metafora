use std::fs::File;
use std::io::prelude::*;

pub fn create_file_with_content(content: &String, filename: &String) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
