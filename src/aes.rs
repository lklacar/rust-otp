use aes_keywrap::Aes256KeyWrap;
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use crypto::symmetriccipher::{BlockDecryptor, BlockEncryptor};

pub fn key_from_string(string: &String) -> [u8; 32] {
    let key = [42u8; 32];
    let kw = Aes256KeyWrap::new(&key);
    let wrapped = kw.encapsulate(string.as_bytes()).unwrap();

    let mut xs: [u8; 32] = [0; 32];
    for i in 0..wrapped.len() {
        xs[i] = wrapped[i];
    }

    return xs;
}

pub fn aes_encrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
    let cipher = AesSafe256Encryptor::new(&key);
    let mut res: Vec<u8> = vec![];

    for chunk in input.chunks(16) {
        let mut output: [u8; 16] = [0; 16];

        cipher.encrypt_block(&chunk, &mut output);

        for byte in output.iter() {
            res.push(*byte);
        }
    }

    return res;
}


pub fn aes_decrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
    let cipher = AesSafe256Decryptor::new(&key);
    let mut res: Vec<u8> = vec![];

    for chunk in input.chunks(16) {
        let mut output: [u8; 16] = [0; 16];
        cipher.decrypt_block(&chunk, &mut output);

        for byte in output.iter() {
            res.push(*byte);
        }
    }

    return res;
}
