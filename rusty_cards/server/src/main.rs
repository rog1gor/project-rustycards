extern crate rusty_cards;
mod server;

use server::Server;

fn main() {
    println!("Hello sunshine :]");
    let server = Server::default();
    server.start();
}
