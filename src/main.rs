#![allow(unused_mut, unused_imports, unused_variables, dead_code, unused_must_use)]
#![feature(collections)]

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
    age: u32,
}

fn main() {
    println!("\nRust JSON and You");
    println!("=================\n");

    // reading_from_file();
    // writing_to_file();

    let user = User {
        name: "Heraclitus".to_string(),
        email: "heraclitus@aol.com".to_string(),
        age: 2550
    };

    // write_user_to_file(user);
    write_user_to_file2(&user);
}

fn write_user_to_file2(user: &User) {
    let json_user: String = encode_user_to_json(user);
    let path = Path::new("src/new_user.json");

    let mut options = OpenOptions::new();
    options.write(true);

    let mut file = match options.open(path) {
      Ok(file) => file,
      Err(..) => panic!("Error opening file!"),
    };

    file.write(json_user.as_bytes());
}

fn write_user_to_file(user: User) {
    let json_user: String = encode_user_to_json(&user);
    let path = Path::new("src/new_user.json");

    let mut options = OpenOptions::new();
    options.write(true);

    let file = options.open(path);

    let file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("Error opening file"),
    };

    // I think I can use a buf writer without knowing the exact size,
    // of my dst_array
    let mut writer = BufWriter::new(&file);
    let json_as_bytes  = json_user.into_bytes();

    // let dst_array_length = json_as_bytes.len() - 1;
    // let box_array = Box::new([0; dst_array_length]);

    let mut dst_array = [0; 60];
    let slice = dst_array.clone_from_slice(&json_as_bytes);

    writer.write_all(&dst_array);
}

fn encode_user_to_json(user: &User) -> String {
    let encoded: String= json::encode(user).unwrap();
    encoded
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
    // println!("{:?}", decoded);

    let encoded = json::encode(&decoded).unwrap();
    // println!("{:?}", encoded);
}
