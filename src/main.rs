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
pub struct UserWithOptionalCity {
    name: String,
    email: String,
    age: u32,
    city_id: Option<i32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct City {
    name: String,
    id: i32,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct SerializedUser {
    name: String,
    email: String,
    age: u32,
    city_name: String
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

    // I want to experiment with Option's for attributes for a User
    let ax = UserWithOptionalCity {
        name: "Anaximander".to_string(),
        email: "anaximander@aol.com".to_string(),
        age: 2625,
        city_id: None,
    };

    // let encoded: String = json::encode(&ax).unwrap();
    // println!("Encoded User with Optional City: {:?}", encoded);
    // and Json encoded handles it perfectly!

    // Zip into a User with City information
    blender(users, cities);
}

fn blender(users: Vec<User>, cities: Vec<City>) {
    let serialized_users = users.iter().map( |user| {
      let city = cities.iter()
        .find(|city| city.id == user.city_id)
        .unwrap();

      let serialized_user = SerializedUser {
        name: user.name.clone(),
        age: user.age.clone(),
        email: user.email.clone(),
        city_name: city.name.clone(),
      };

      serialized_user
    });

    for user in serialized_users {
      let encoded: String = json::encode(&user).unwrap();
      println!("{:?}", encoded);
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
    let encoded: String = json::encode(user).unwrap();
    encoded
}

