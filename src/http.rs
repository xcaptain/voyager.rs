pub mod handler;
pub mod mux;
pub mod server;

use handler::Handler;
use server::Server;

pub fn listen_and_serve(addr: String, handler: Handler) {
    let server = Server::new(addr, handler);
    return server.listen_and_serve();
}
