use postgres::Error as PostgressError;
use postgres::{Client, NoTls};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;


#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>, 
    name: String,
    email: String
}


#[macro_use]
extern crate serde_derive;
fn main() {
    println!("Hello, world!");
}
