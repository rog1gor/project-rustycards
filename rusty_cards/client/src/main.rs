extern crate rusty_cards;
mod action;
mod p2p_client;

use std::{ io, net::SocketAddr };

use p2p_client::Client;

// Sending a message that should explain what kind of message is expected
// and reading that message from stdin
fn provide_input(msg: &str) -> String {
    println!("{msg}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Sending a message that we're expecting a socket address of a server
// reading that socket addres from stdin and parsing it into SocketAddress
// If the address is incorrect, then returns a default socket address
fn provide_server() -> SocketAddr {
    let input = provide_input("Provide a socket that the server will be binded to");
    let socket_addr: SocketAddr = match input.parse() {
        Ok(addr) => {
            println!("Binding server to {input}.");
            addr
        },
        Err(_) => {
            println!("Incorrect address! Binding server to default...");
            Client::default_server()
        },
    };
    socket_addr
}

// Sending a message that we're expecting a socket address of a TcpListener
// reading that socket addres from stdin and parsing it into SocketAddress
// If the address is incorrect, then returns a default socket address
fn provide_listener() -> SocketAddr {
    let input = provide_input("Provide a socket that the listener will be binded to");
    let socket_addr: SocketAddr = match input.parse() {
        Ok(addr) => {
            println!("Binding TcpListener to {input}.");
            addr
        },
        Err(_) => {
            println!("Incorrect address! Binding TcpListener to default...");
            Client::default_listener()
        },
    };
    socket_addr
}

fn main() -> io::Result<()> {
    println!("Hello angel :]");

    // Constructing a client (default or custom)
    let mut client = Client::default();
    let input = provide_input("Do you want to use default options (listed above)? If so, type: \'y\':");
    if input.to_lowercase() != "y" {
        println!("You chose not to use default configurations");
        let server_socket = provide_server();
        let listener_socket = provide_listener();
        client = Client::new(server_socket, listener_socket);
    }

    // Looking for opponent or leaving a game
    println!("Client ready to look for an opponent ^_^");
    let mut exit_game: bool = false;
    while !exit_game {
        let input = provide_input("Do you want to look for an opponent? If so type \'y\':");

        if input.to_lowercase() == "y" {
            client.start()?;
        } else {
            println!("Understood. Exiting the game. Have a great day n_n");
            exit_game = true;
        }
    }

    Ok(())
}
