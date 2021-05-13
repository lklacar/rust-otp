use aes_keywrap::Aes256KeyWrap;
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use crypto::symmetriccipher::{BlockDecryptor, BlockEncryptor};

const KEY_SIZE: usize = 32;
pub const CHUNK_SIZE: usize = 16;

pub fn key_from_string(string: &String) -> [u8; 32] {
    let key = [42u8; KEY_SIZE];
    let wrapper = Aes256KeyWrap::new(&key);
    let wrapped = wrapper.encapsulate(string.as_bytes()).unwrap();
    
    let mut xs: [u8; KEY_SIZE] = [0; KEY_SIZE];
    for i in 0..wrapped.len() {
        xs[i] = wrapped[i];
    }

    return xs;
}

pub fn aes_encrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
    let cipher = AesSafe256Encryptor::new(&key);

    return input.chunks(CHUNK_SIZE).flat_map(|chunk| {
        let mut output: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
        cipher.encrypt_block(&chunk, &mut output);
        return output.to_vec().into_iter();
    }).collect();
}

pub fn aes_decrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
    let cipher = AesSafe256Decryptor::new(&key);

    return input.chunks(CHUNK_SIZE).flat_map(|chunk| {
        let mut output: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
        cipher.decrypt_block(&chunk, &mut output);
        return output.to_vec().into_iter();
    }).collect();
}
