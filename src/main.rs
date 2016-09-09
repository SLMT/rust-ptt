
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;

mod telnet;
mod terminal;

use telnet::TelnetConnection;
use terminal::Terminal;

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

    // TODO: These should be a part of program options
    let width = 80;
    let height = 24;

    // Initialize telnet
    let mut telnet_conn = TelnetConnection::new(&mut stream, Some(resize_term));

    // TODO: Initialize terminal for the user
    let term = initialize_term(width, height);

    // TODO: Initialize many things written in start_client() of mbbsd.c

    // TODO: Let the user login

    // TODO: Initialize other things after logging in

    // TODO: Enter main menu

    // XXX: Debug: receive bytes
    for byte in stream.bytes() {
        telnet_conn.process(byte.unwrap());
    }
}

fn resize_term(width: u32, height: u32) {
    println!("Width: {}, Height: {}", width, height);
}

// TODO: Terminal mode ?
fn initialize_term(width: usize, height: usize) -> Terminal {
    // TODO: System initialization for the terminal (register signal for resizing)

    // Create a terminal object
    let term = Terminal::new(width, height);

    // TODO: Resize the terminal
    // We really need to do this ? Even we have already passed the arguments to the constructor ?

    // TODO: Send resizing signal

    term
}

fn login_query() {
    // TODO: Print the welcome screen

    // TODO: Do other things of login_query() in mbbsd.c
}
