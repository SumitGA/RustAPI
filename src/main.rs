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

const DB_URL: &str = env!("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";


#[macro_use]
extern crate serde_derive;

fn main() {
}
