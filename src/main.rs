#![allow(unused_mut, unused_imports, unused_variables)]

extern crate rustc_serialize;
extern crate type_printer;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct User {
    name: String,
    email: String,
    age: u8,
}

fn main() {
    println!("\nRust JSON and You");
    println!("=================\n");

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

    match file.read_to_string(&mut s) {
        Ok(_) => print!("{} contains:\n{}", display, s),
        Err(why) => panic!(
            "couldn't read {}: {}", display,
            Error::description(&why)
        ),
    }

    // ==========================================

    let user = User {
      name: "Frank Stella".to_string(),
      email: "frankstella@aol.com".to_string(),
      age: 79,
    };

    let encoded = json::encode(&user).unwrap();
    println!("{:?}", encoded);

    let decoded: User = json::decode(&encoded).unwrap();
    println!("{:?}", decoded);
}
