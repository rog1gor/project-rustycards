use log::{trace, warn};
use std::{
    io::{self, Read, Write},
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
    sync::{atomic::AtomicBool, Mutex},
};

use rusty_cards::Handshake;

// Class that stores the stream of the client and socket address that the client sent to the server
struct ClientsAddr {
    stream: TcpStream,
    listener_addr: SocketAddr,
}

impl ClientsAddr {
    fn new(stream: TcpStream, listener_addr: SocketAddr) -> Self {
        ClientsAddr {
            stream,
            listener_addr,
        }
    }

    fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    fn get_listener_addr(&self) -> SocketAddr {
        self.listener_addr
    }
}

pub struct Server {
    tcp_listener: TcpListener,
    running: AtomicBool,
}

impl Default for Server {
    fn default() -> Self {
        println!("Binding server to 127.0.0.1:8888");
        let tcp_listener = TcpListener::bind("127.0.0.1:8888").unwrap();
        Server {
            tcp_listener,
            running: AtomicBool::new(true),
        }
    }
}

impl Server {
    pub fn start(&self) {
        println!("Starting the server on port {}...", self.port());
        let players = Mutex::new(Vec::new());
        std::thread::scope(|s| {
            for stream in self.tcp_listener.incoming() {
                trace!("Client appeared!");
                let stream = stream.unwrap();
                if !self.running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

                s.spawn(|| match Self::handle_connection(&players, stream) {
                    Ok(_) => (),
                    Err(e) => warn!("Error: {e}"),
                });
            }
        });
    }

    // Handles messages from the clients
    // If there are no other players looking for the game, then it stores ClientsAddr
    // Otherwise it takes the ClientsAddr that appeared earlier and handshakes them
    fn handle_connection(
        players: &Mutex<Vec<ClientsAddr>>,
        mut stream: TcpStream,
    ) -> io::Result<()> {
        trace!("Handling connection...");
        let mut buffer = [0; 1024];
        let num_bytes = stream.read(&mut buffer)?;
        trace!("Parsing message...");

        let msg: Handshake = serde_json::from_slice(&buffer[..num_bytes]).unwrap();
        match msg {
            Handshake::Ready(s) => {
                let curr_player = ClientsAddr::new(stream, s);
                println!("Clients TcpListener socket: {}", s);
                let mut players = players.lock().unwrap();
                if !players.is_empty() {
                    trace!("There is another player looking for the opponent!");
                    Self::handshake_players(players.remove(0), curr_player)?;
                } else {
                    trace!("No other player looking for the game :c Adding to the queue.");
                    players.push(curr_player);
                }
            }
            _ => {
                warn!("Unexpected message @_@");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Incorrect type of the message. Aborting this stream",
                ));
            }
        }

        Ok(())
    }

    // Handshakes two players:
    //
    // Sending a message to a player one that they should
    // wait for a password from the player two
    //.
    // Sending a message to a player two that they shold
    // send a message with a password to an address provided
    // by the player one eariler
    //
    // For now the password is always 'KOZA'
    fn handshake_players(player1: ClientsAddr, player2: ClientsAddr) -> io::Result<()> {
        trace!("Handshaking players...");
        // Create game state
        let mut side1 = game::player::Player::me();
        side1.shuffle_deck();
        let mut side2 = game::player::Player::opponent();
        side2.shuffle_deck();

        let mut game_state1 = game::GameState::new();
        game_state1.change_player(game::player::Side::Me, &side1);
        game_state1.change_player(game::player::Side::Opponent, &side2);
        game_state1.set_turn(true);

        side1.set_side(game::player::Side::Opponent);
        side2.set_side(game::player::Side::Me);
        let mut game_state2 = game::GameState::new();
        game_state2.change_player(game::player::Side::Me, &side2);
        game_state2.change_player(game::player::Side::Opponent, &side1);
        game_state2.set_turn(false);

        // Send info of player2 to to player1
        let msg1 = Handshake::Send(player2.get_listener_addr(), "KOZA".to_string(), game_state1);
        let serialized_msg1 = serde_json::to_vec(&msg1).unwrap();
        player1.get_stream().write_all(&serialized_msg1)?;
        trace!("Sending {} bytes to player1", serialized_msg1.len());

        // Send info of player1 to to player2
        let msg2 = Handshake::Wait("KOZA".to_string(), game_state2);
        let serialized_msg2 = serde_json::to_vec(&msg2).unwrap();
        player2.get_stream().write_all(&serialized_msg2)?;
        trace!("Sending {} bytes to player2", serialized_msg2.len());

        player1.get_stream().shutdown(Shutdown::Both).unwrap();
        player2.get_stream().shutdown(Shutdown::Both).unwrap();

        Ok(())
    }

    pub fn port(&self) -> u16 {
        self.tcp_listener.local_addr().unwrap().port()
    }
}
