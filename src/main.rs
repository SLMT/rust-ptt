
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;

mod telnet;

use telnet::TelnetConnection;

fn main() {
    // TODO: Read program arguments

    // TODO: Should we use shared memory ?

    // TODO: Initialize a connection listener according to settings
    // TODO: Need different types of connection
    start_tcp_server(54321);
}

fn start_tcp_server(port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

    for result in listener.incoming() {
        match result {
            Ok(stream) => {
                thread::spawn(move || {
                    start_tcp_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn start_tcp_connection(mut stream: TcpStream) {
    // Print IP Address
    let addr = stream.peer_addr().unwrap();
    println!("Start a connection to {}", addr);

    // Initialize telnet
    let mut telnet_conn = TelnetConnection::new(&mut stream, Some(resize_term));

    // TODO: Initialize terminal for the user

    // TODO: Let the user login

    // TODO: Enter main menu

    // XXX: Debug: receive bytes
    for byte in stream.bytes() {
        telnet_conn.process(byte.unwrap());
    }
}

fn resize_term(width: u32, height: u32) {
    println!("Width: {}, Height: {}", width, height);
}
