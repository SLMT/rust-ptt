
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;

mod telnet;

fn main() {
    // TODO: Read program arguments

    // TODO: Should we use shared memory ?

    // TODO: Initialize a connection listener according to settings
    // TODO: Need different types of connection
    start_tcp_server(54321);

    // TODO: Initialize terminal for the user

    // TODO: Let the user login

    // TODO: Enter main menu

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
    telnet::initialize(&mut stream);

    // Receive bytes
    for byte in stream.bytes() {
        println!("{}", byte.unwrap());
    }
}
