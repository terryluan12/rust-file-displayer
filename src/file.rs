
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct File {
    file_name: String,
    source_name: String,
    extension: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(file_name: String, source_name: String, extension: String, data: Vec<u8>) -> File {
        File{file_name, source_name, extension, data}
    }
}

impl File {
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }
    pub fn get_extension(&self) -> &String {
        &self.extension
    }
}