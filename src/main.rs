use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("127.0.0.1:6379") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 6379");

            let msg = b"HTTP/1.1 200 OK\r\n\r\n";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0u8; 16]; // using 16 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply OK: {}", text);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}