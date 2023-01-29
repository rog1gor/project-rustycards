use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

extern crate game;
use game::GameState;

#[derive(Debug, Serialize, Deserialize)]
pub enum Handshake {
    // Info that client sends to the server when ready to play
    Ready(SocketAddr),
    // Info from server that client should send password (String)
    // to SocketAddr to establish p2p connection with other player
    Send(SocketAddr, String, GameState),
    // Info from server that client should wait for password (String)
    // from other client to establish p2p connection with them
    Wait(String, GameState),
    // Message send from client to client to connect with password
    P2P(String),
    None,
}

impl PartialEq for Handshake {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Handshake::Ready(_), Handshake::Ready(_)) => true,
            (Handshake::Send(addr1, password1, _), Handshake::Send(addr2, password2, _)) => {
                (addr1 == addr2) && (password1 == password2)
            }
            (Handshake::Wait(password1, _), Handshake::Wait(password2, _)) => {
                password1 == password2
            }
            (Handshake::P2P(password1), Handshake::P2P(password2)) => password1 == password2,
            _ => false,
        }
    }
}
