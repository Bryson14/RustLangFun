use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let port = "127.0.0.1:8080";
    let listener = TcpListener::bind(port).expect("Error binding to the port");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection established! !!");
    }
}

fn handle_connection(mut stream: TcpStream) {}
