use log::{info, trace};
use nom::{bytes::complete::tag, character::complete::digit1, IResult};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::action::Action;

// Prints given message, that should describe what kind of
// input is expected then reads the input from stdin
pub fn provide_input(msg: &str) -> String {
    println!("{msg}");
    let mut input = String::new();
    while let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error {} occured while reading input. Retrying...", e);
    }
    input.trim().to_lowercase()
}

// Reads socket address of the server from stdin and parses it to SocketAddr
// Returns default server's SocketAddr if the input from stdin is not a valid SocketAddress
pub fn provide_server() -> SocketAddr {
    let input = provide_input("Provide a socket that the server will be bound to:");
    let socket_addr: SocketAddr = match input.parse() {
        Ok(addr) => {
            info!("Server's address is set to {input}.");
            addr
        }
        Err(_) => {
            println!("Incorrect address!");
            provide_server()
        }
    };
    socket_addr
}

// Reads socket address of the listener from stdin, that the client will be bound to and parses it to SocketAddr
// Returns default listener's SocketAddr if the input from stdin is not a valid SocketAddr
pub fn provide_listener() -> SocketAddr {
    let input = provide_input("Provide a socket that the listener will be bound to:");
    let socket_addr: SocketAddr = match input.parse() {
        Ok(addr) => {
            info!("Listener's address is set to {input}.");
            addr
        }
        Err(_) => {
            println!("Incorrect address!");
            provide_listener()
        }
    };
    socket_addr
}

// Functions that reads ingame actions from stdin and parses them
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
        provide_input("Provide action that you want to perform (help for list of actions):");

    if let Ok((args, _)) = get_play_card(&input) {
        if let Ok((n2, n1)) = get_play_args(args.trim()) {
            if let Ok((rest, n2)) = get_play_args(n2.trim()) {
                if rest.trim() != "" {
                    return Action::Help;
                }

                let num1 = if let Ok(n) = n1.trim().parse::<usize>() {
                    n
                } else {
                    println!("Couldn't parse the first argument");
                    return Action::Help;
                };
                let num2 = if let Ok(n) = n2.trim().parse::<usize>() {
                    n
                } else {
                    println!("Couldn't parse the second argument");
                    return Action::Help;
                };

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

// Serializes a message of type T and sends it to a given stream
pub fn send_msg<T>(stream: &mut TcpStream, msg: T) -> io::Result<()>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let serialized_msg = serde_json::to_vec(&msg)?;
    info!("Sending a message: {:?}", serialized_msg);
    stream.write_all(&serialized_msg)?;

    Ok(())
}

// Tries to read a message from stream until something appear on it
// After that, reads the message and returns number of bytes read
pub fn read_msg(stream: &mut TcpStream, buffer: &mut [u8]) -> usize {
    loop {
        if let Ok(num_bytes) = stream.read(buffer) {
            info!("Message finally appeared");
            return num_bytes;
        }
        trace!("There is no message on the stream")
    }
}
