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
    city_id: i32,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct City {
    name: String,
    id: i32,
}

fn main() {
    println!("\nRust JSON and You");
    println!("=================\n");

    let ephesus = City {
        name: "Ephesus".to_string(),
        id: 1,
    };

    let sinop = City {
        name: "Sinop".to_string(),
        id: 2,
    };

    let hera = User {
        name: "Heraclitus".to_string(),
        email: "heraclitus@aol.com".to_string(),
        age: 2550,
        city_id: 1,
    };

    let dio = User {
        name: "Diogenes".to_string(),
        email: "diogenes@aol.com".to_string(),
        age: 2427,
        city_id: 2,

    };

    // I should figure out how to have this append to be valid json
    // write_user_to_file(&hera);
    // write_user_to_file(&dio);

    let users = vec![hera, dio];
    let cities = vec![ephesus, sinop];
    blender(users, cities);
}

fn blender(users: Vec<User>, cities: Vec<City>) {
    for user in users {
      println!("User: {:?}", user);
    }

    for city in cities {
      println!("City: {:?}", city);
    }
}

fn write_user_to_file(user: &User) {
    let json_user: String = encode_user_to_json(user);
    let path = Path::new("src/new_user.json");

    let mut options = OpenOptions::new();
    options.write(true);
    options.append(true);

    let mut file = match options.open(path) {
      Ok(file) => file,
      Err(..) => panic!("Error opening file!"),
    };

    file.write(json_user.as_bytes());
}

fn encode_user_to_json(user: &User) -> String {
    let encoded: String= json::encode(user).unwrap();
    encoded
}

