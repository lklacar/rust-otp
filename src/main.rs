#[macro_use]
extern crate clap;

use std::{fs, process};
use std::fs::File;
use std::io::{Read, Write};

use aes_keywrap::Aes256KeyWrap;
use clap::App;
use crypto::aessafe::AesSafe256Encryptor;
use crypto::symmetriccipher::BlockEncryptor;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use sha256::digest;

use crate::aes::key_from_string;
use crate::files::{read_bytes, write_bytes};
use crate::otp::{otp_decrypt, otp_encrypt};

mod files;
mod aes;
mod otp;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if (!matches.is_present("encrypt") && !matches.is_present("decrypt")) || (matches.is_present("encrypt") && matches.is_present("decrypt")) {
        println!("Choose either encryption or decryption options. Use --help for more information.");
        process::exit(1);
    }

    let filename = match matches.value_of("INPUT") {
        Some(input) => String::from(input),
        None => {
            println!("Please provide INPUT parameter. See --help for more information");
            process::exit(1);
        }
    };

    if matches.is_present("encrypt") {
        otp_encrypt(&filename);
    }

    if matches.is_present("decrypt") {
        let keypath = match matches.value_of("key") {
            Some(path) => { String::from(path) }
            None => {
                println!("Please provide key for decryption. See --help for more information.");
                process::exit(1);
            }
        };

        otp_decrypt(&filename, &keypath);
    }
}
