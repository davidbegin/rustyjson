extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct User {
    name: String,
    email: String,
    age: u8,
}

fn main() {
    println!("\nRust JSON and You");
    println!("=================\n");

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
