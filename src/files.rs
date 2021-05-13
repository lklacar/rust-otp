use std::{fs, process};
use std::fs::{File, Metadata};
use std::io::{Error, Read, Write};

pub fn read_bytes(filename: &String) -> Vec<u8> {
    let file_metadata = match fs::metadata(&filename) {
        Ok(x) => { x }
        Err(_) => {
            eprintln!("File '{}' not valid.", filename);
            process::exit(1);
        }
    };

    let mut file = File::open(&filename).expect("File not found.");
    let mut buffer = vec![0; file_metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overload");
    return buffer;
}

pub fn write_bytes(filename: &String, bytes: &Vec<u8>) {
    let mut file = File::create(&filename).expect("Cannot create encrypted file");
    file.write(bytes);
}
