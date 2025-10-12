use std::fs::File;
use std::io::Write;

pub fn create_file_with_content(content: &Vec<u8>, filename: &String) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content)?;
    Ok(())
}


