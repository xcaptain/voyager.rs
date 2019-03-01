pub mod handler;
pub mod mux;
pub mod request;
pub mod response;
pub mod server;

// use handler::Handler;
use mux::Mux;
use server::Server;

use request::Request;
use response::ResponseWriter;

pub fn listen_and_serve<T: Fn(&ResponseWriter, &Request) + Clone>(addr: String, m: Mux<T>) {
    let server = Server::new(addr, m);
    server.listen_and_serve()
}
