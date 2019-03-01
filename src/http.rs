pub mod handler;
pub mod mux;
pub mod request;
pub mod response;
pub mod server;

// use handler::Handler;
use mux::Mux;
use server::Server;

pub fn listen_and_serve(addr: String, m: Mux) {
    let server = Server::new(addr, m);
    server.listen_and_serve()
}
