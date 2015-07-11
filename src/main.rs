#![allow(unused_mut, unused_imports, unused_variables, dead_code)]

extern crate rustc_serialize;
extern crate type_printer;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufWriter;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct User {
    name: String,
    email: String,
    age: u8,
}

fn main() {
    println!("\nRust JSON and You");
    println!("=================\n");

    // reading_from_file();
    writing_to_file();
}

fn writing_to_file() {
    let path = Path::new("src/new_user.json");

    let mut options = OpenOptions::new();
    options.write(true);

    let file = options.open(path);

    let file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("Error opening file"),
    };

    let mut writer = BufWriter::new(&file);
    writer.write_all(b"this will json one day\n");
}

fn reading_from_file() {
    let path = Path::new("src/user.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            panic!(
                "couldn't open {}: {}",
                display, Error::description(&why)
            )
        }
    };

    let mut s = String::new();
    let file_to_string_result = file.read_to_string(&mut s);

    match file_to_string_result {
        Ok(file) => {
            println!("File to String");
        },
        Err(e) => {
            println!("Error!");
        }
    }

    let decoded: User = json::decode(&s).unwrap();
    println!("{:?}", decoded);

    let encoded = json::encode(&decoded).unwrap();
    println!("{:?}", encoded);
}
