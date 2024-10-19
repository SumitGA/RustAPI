use postgres::Error as PostgressError;
use postgres::{Client, NoTls};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

const DB_URL: &str = env!("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

#[macro_use]
extern crate serde_derive;

fn main() {
    // Set Database
    if let Err(_) = set_database() {
        println!("Error setting database");
        return;
    }

    // start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwarp();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

// db setup
fn set_database() -> Result<(), PostgressError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
         CREATE TABLE IF NOT EXISTS users(
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL, 
            email VARCHAR NOT NULL
         )
    ",
    )?;
    Ok(())
}

//Get id from request URL
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

//deserialize user from reqeust body without id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

//handle requests
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(r),
                r if r.starts_with("GET /user") => handle_get_request(r),
                r if r.starts_with("GET /users") => handle_get_all_request(r),
                r if r.starts_with("PUT /users") => handle_post_request(r),
                r if r.starts_with("DELETE /users") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            }

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}
