use std::{
    fs,
    io::{self, Read},
};

pub fn read_bytes(file_path: Option<&str>) -> io::Result<Vec<u8>> {
    match file_path {
        Some(file_path) => fs::read(file_path),

        None => {
            let mut buffer: Vec<u8> = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
}

pub fn read_string(file_path: Option<&str>) -> io::Result<String> {
    match file_path {
        Some(file_path) => fs::read_to_string(file_path),
        None => {
            let mut buffer: String = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}
