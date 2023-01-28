use nom::{bytes::complete::tag, character::complete::digit1, IResult};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    net::SocketAddr,
    net::TcpStream,
};

use crate::action::Action;
use crate::Client;

// Sending a message that should explain what kind of message is expected
// and reading that message from stdin
pub fn provide_input(msg: &str) -> String {
    println!("{msg}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase().to_string()
}

// Sending a message that we're expecting a socket address of a server
// reading that socket addres from stdin and parsing it into SocketAddress
// If the address is incorrect, then returns a default socket address
pub fn provide_server() -> SocketAddr {
    let input = provide_input("Provide a socket that the server will be binded to");
    let socket_addr: SocketAddr = match input.parse() {
        Ok(addr) => {
            println!("Binding server to {input}.");
            addr
        }
        Err(_) => {
            println!("Incorrect address! Binding server to default...");
            Client::default_server()
        }
    };
    socket_addr
}

// Sending a message that we're expecting a socket address of a TcpListener
// reading that socket addres from stdin and parsing it into SocketAddress
// If the address is incorrect, then returns a default socket address
pub fn provide_listener() -> SocketAddr {
    let input = provide_input("Provide a socket that the listener will be binded to");
    let socket_addr: SocketAddr = match input.parse() {
        Ok(addr) => {
            println!("Binding TcpListener to {input}.");
            addr
        }
        Err(_) => {
            println!("Incorrect address! Binding TcpListener to default...");
            Client::default_listener()
        }
    };
    socket_addr
}

fn get_play_card(i: &str) -> IResult<&str, &str> {
    tag("play card")(i)
}

fn get_play_args(i: &str) -> IResult<&str, &str> {
    digit1(i)
}

fn get_end_turn(i: &str) -> IResult<&str, &str> {
    tag("end turn")(i)
}

fn get_help(i: &str) -> IResult<&str, &str> {
    tag("help")(i)
}

pub fn provide_action() -> Action {
    let input =
        provide_input("Provide action that you want to perform (help for list of actions).");
    
    if let Ok((args, _)) = get_play_card(&input) {
        if let Ok((n2, n1)) = get_play_args(args.trim()) {
            if let Ok((rest, n2)) = get_play_args(n2.trim()) {
                if rest.trim() != "" {
                    return Action::Help;
                }
                let num1 = n1.trim().parse::<usize>().unwrap();
                let num2 = n2.trim().parse::<usize>().unwrap();
                return Action::PlayCard(num1, num2);
            } else {
                return Action::Help;
            }
        } else {
            return Action::Help;
        }
    } else if let Ok((_, _)) = get_end_turn(&input) {
        return Action::EndTurn;
    } else if let Ok((_, _)) = get_help(&input) {
        return Action::Help;
    }
    Action::Help
}

// Serializes a message of type T and puts it to a given stream
pub fn send_msg<T>(stream: &mut TcpStream, msg: T) -> io::Result<()>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let serialized_msg = serde_json::to_vec(&msg)?;
    println!("Sending a message: {:?}", serialized_msg);
    stream.write_all(&serialized_msg)?;

    Ok(())
}
