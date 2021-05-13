use rand::{RngCore, SeedableRng};
use rand_hc::Hc128Rng;

pub fn rand_array(length: u32) -> Vec<u8> {
    let mut random_generator: Hc128Rng = Hc128Rng::from_entropy();
    return (0..length / 4).flat_map(|_| {
        let rand = random_generator.next_u32();
        let rand_bytes = rand.to_be_bytes();
        return rand_bytes.to_vec().into_iter();
    }).collect();
}

pub fn otp(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    return (0..input.len()).map(|i| {
        let input_byte = input[i];
        let key_byte = key[i];
        let otp_byte = input_byte ^ key_byte;
        return otp_byte;
    }).collect();
}
