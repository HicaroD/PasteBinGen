use std::fs;

pub fn get_file_as_string(path: String) -> String {
    fs::read_to_string(path).expect("Unable to read file to string")
}
