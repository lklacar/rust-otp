use std::fs;
use std::fs::File;
use std::io::{Read, Write};

pub fn read_bytes(filename: &String) -> Vec<u8> {
    let mut file = File::open(&filename).expect("File not found");
    let file_metadata = fs::metadata(&filename).expect("Unable to read file metadata");
    let mut buffer = vec![0; file_metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overload");
    return buffer;
}

pub fn write_bytes(filename: &String, bytes: &Vec<u8>) {
    let mut file = File::create(&filename).expect("Cannot create encrypted file");
    file.write(bytes);
}
