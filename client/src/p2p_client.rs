use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::io::{self, prelude::*, BufReader, Read, Write};
use std::sync::atomic::AtomicBool;

use crate::action::Action;

pub struct Client {
    tcp_listener: TcpListener,
    running: AtomicBool,
}

impl Default for Client {
    fn default() -> Self {
        let tcp_listener = TcpListener::bind("127.0.0.1:8888").unwrap();
        Client {
            tcp_listener,
            running: AtomicBool::new(true),
        }
    }
}

impl Client {
    pub fn new(port: u16) -> Self {
        let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        let tcp_listener = TcpListener::bind(socket_addr).unwrap();
        Client {
            tcp_listener,
            running: AtomicBool::new(true),
        }
    }

    pub fn start(&self) {
        println!("Starting server...");
        for stream in self.tcp_listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_action(stream).unwrap();
        }
    }

    pub fn handle_action(mut stream: TcpStream) -> io::Result<()> {
        println!("Message came!");
        let mut buf_reader = BufReader::new(&mut stream);
        let mut buffer = Vec::new();

        println!("Reading the message...");
        buf_reader.read_to_end(&mut buffer)?;

        println!("Parsing the message...");
        let action: Action = serde_json::from_slice(&buffer)?;
        println!("Action from another client: {:?}", action);

        Ok(())
    }

    pub fn send_action(mut stream: TcpStream) -> io::Result<()> {
        let action = Action::Handshake;
        let serialized_action = serde_json::to_vec(&action)?;
        stream.write_all(&serialized_action)?;

        Ok(())
    }

    pub fn stop(&self) {
        // We set the flag and make a new connection to make sure that the server checks the flag
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        TcpStream::connect(("127.0.0.1", self.port())).unwrap();

        // TODO: close all streams using `shutdown`
    }

    pub fn port(&self) -> u16 {
        self.tcp_listener.local_addr().unwrap().port()
    }
}