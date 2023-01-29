extern crate game;
extern crate rusty_cards;
mod server;

use server::Server;

fn main() {
    println!("Hello Game Master :3");
    let server = Server::default();
    server.start();
}
