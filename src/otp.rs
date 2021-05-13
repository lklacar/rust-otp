use std::process;

use crypto::aessafe::AesSafe256Encryptor;
use rand::{RngCore, SeedableRng};
use rand_hc::Hc128Rng;

use crate::aes::{aes_decrypt, aes_encrypt, CHUNK_SIZE, key_from_string};
use crate::files::{read_bytes, write_bytes};

fn rand_array(length: u32) -> Vec<u8> {
    let mut random_generator: Hc128Rng = Hc128Rng::from_entropy();
    let mut result: Vec<u8> = vec![];
    let limit = (length + CHUNK_SIZE as u32 - length % CHUNK_SIZE as u32) / 4;
    for _i in 0..limit {
        let rand = random_generator.next_u32();
        let rand_bytes = rand.to_be_bytes();
        for byte in &rand_bytes {
            result.push(*byte);
        }
    }

    return result;
}

fn otp(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for i in 0..input.len() {
        let input_byte = input[i];
        let key_byte = key[i];
        let otp_byte = input_byte ^ key_byte;
        result.push(otp_byte);
    }
    return result;
}

pub fn otp_encrypt(input_file_name: &String) {
    let file = read_bytes(&input_file_name);

    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let key = key_from_string(&pass);
    let secure_bytes = rand_array(file.len() as u32);

    let encrypted_key = aes_encrypt(&secure_bytes, &key);
    let encrypted = otp(&file, &secure_bytes);

    write_bytes(&format_args!("{filename}.enc", filename = input_file_name).to_string(), &encrypted);
    write_bytes(&format_args!("{filename}.key", filename = input_file_name).to_string(), &encrypted_key);
}

pub fn otp_decrypt(input_file_name: &String, keypath: &String) {
    let file = read_bytes(&input_file_name);

    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let secure_bytes = read_bytes(&keypath);
    let key = key_from_string(&pass);
    let decrypted_key = aes_decrypt(&secure_bytes, &key);

    let encrypted = otp(&file, &decrypted_key);
    write_bytes(&format_args!("{filename}.dec", filename = input_file_name).to_string(), &encrypted);
}
