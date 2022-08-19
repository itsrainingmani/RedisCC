#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    // Loop to read input from client and send a response back
    loop {
        // Initialize a buffer of 50 bytes
        let mut data = [0 as u8; 50];
        match stream.read(&mut data) {
            Ok(_size) => {
                // Send back PONG regardless of what's been sent
                stream.write(b"+PONG\r\n").unwrap();
            }
            Err(_) => {}
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    match listener.accept() {
        Ok((socket, addr)) => {
            println!("accepted new client: {:?}", addr);
            handle_connection(socket);
        }
        Err(e) => println!("couldn't accept client: {:?}", e),
    }
}
