#[macro_use]
extern crate clap;

use std::{fs, process};
use std::fs::File;
use std::io::{Read, Write};

use aes_keywrap::Aes256KeyWrap;
use clap::{AppSettings, Clap};
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

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::E(params) => otp_encrypt(&params.input),
        SubCommand::D(params) => otp_decrypt(&params.input, &params.key)
    }
}
