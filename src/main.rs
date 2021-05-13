extern crate clap;

use clap::{AppSettings, Clap};

use crate::aes::{aes_decrypt, aes_encrypt, CHUNK_SIZE, key_from_string};
use crate::files::{read_bytes, write_bytes};
use crate::otp::{otp, rand_array};

mod files;
mod aes;
mod otp;

/// CLI utility for OTP+AES encryption
#[derive(Clap)]
#[clap(version = "1.0", author = "Luka Klačar <luka@qubit.rs>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    E(Encrypt),
    D(Decrypt),
}

/// Encrypt file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Encrypt {
    /// Input file
    input: String,
}

///  Decrypt file
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Decrypt {
    /// Input file
    input: String,

    /// OTP Key
    key: String,
}

pub fn otp_encrypt(input_file_name: &String) {
    let file_data = read_bytes(&input_file_name);

    let password = rpassword::prompt_password_stdout("Password: ").unwrap();
    let aes_key = key_from_string(&password);

    let random_bytes = rand_array((file_data.len() + CHUNK_SIZE - file_data.len() % CHUNK_SIZE) as u32);

    let encrypted_otp_key = aes_encrypt(&random_bytes, &aes_key);
    let encrypted_data = otp(&file_data, &random_bytes);

    write_bytes(&format_args!("{filename}.enc", filename = input_file_name).to_string(), &encrypted_data);
    write_bytes(&format_args!("{filename}.key", filename = input_file_name).to_string(), &encrypted_otp_key);
}

pub fn otp_decrypt(input_file_name: &String, keypath: &String) {
    let file_data = read_bytes(&input_file_name);

    let password = rpassword::prompt_password_stdout("Password: ").unwrap();
    let encrypted_otp_key = read_bytes(&keypath);
    let aes_key = key_from_string(&password);
    let decrypted_otp_key = aes_decrypt(&encrypted_otp_key, &aes_key);

    let encrypted = otp(&file_data, &decrypted_otp_key);
    write_bytes(&format_args!("{filename}.dec", filename = input_file_name).to_string(), &encrypted);
}


fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::E(params) => otp_encrypt(&params.input),
        SubCommand::D(params) => otp_decrypt(&params.input, &params.key)
    }
}
