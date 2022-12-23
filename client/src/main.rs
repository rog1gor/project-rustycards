mod card;
use card::Minion;
mod action;
mod p2p_client;

use std::io::{self, Write};
use std::net::{ToSocketAddrs, SocketAddr, TcpStream};
use std::thread;

use p2p_client::Client;
use action::Action;

fn main() {
    println!("Hello :]");

    // Get port from the stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    
    // Bind the client to the port
    let mut client = Client::default();
    let num: Result<u16, _> = input.parse();
    if num.is_ok() {
        let ok_num = num.unwrap();
        client = Client::new(ok_num);
        println!("Binding to a port: {}...", ok_num); 
    } else {
        println!("Input is not a correct port. Binding to default (8888) ...");
    }

    // Waiting for messages
    println!("Waiting for messages...");
    let action_handler = thread::spawn(move || client.start());

    // Get socket from the stdin
    println!("Provide opponent scoket:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    // Binding opponent socket
    let opponent_socket: Vec<SocketAddr> = match input.to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(error) => {
            panic!("Incorrect socket provided!");
        }
    };
    println!("Opponent binded to a socket: {:?}", opponent_socket);

    // Sending a message
    let mut opponent_stream = TcpStream::connect(opponent_socket[0]).unwrap();
    Client::send_action(opponent_stream);

    action_handler.join();
}
