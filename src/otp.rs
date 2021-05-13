use rand::{RngCore, SeedableRng};
use rand_hc::Hc128Rng;

use crate::aes::{aes_decrypt, aes_encrypt, CHUNK_SIZE, key_from_string};
use crate::files::{read_bytes, write_bytes};

fn rand_array(length: u32) -> Vec<u8> {
    let mut random_generator: Hc128Rng = Hc128Rng::from_entropy();
    let limit = (length + CHUNK_SIZE as u32 - length % CHUNK_SIZE as u32) / 4;
    return (0..limit).flat_map(|_| {
        let rand = random_generator.next_u32();
        let rand_bytes = rand.to_be_bytes();
        return rand_bytes.to_vec().into_iter();
    }).collect();
}

fn otp(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    return (0..input.len()).map(|i| {
        let input_byte = input[i];
        let key_byte = key[i];
        let otp_byte = input_byte ^ key_byte;
        return otp_byte;
    }).collect();
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
