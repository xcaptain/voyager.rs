use crate::http::handler::Handler;

pub struct Server {
    pub addr: String,
    pub handler: Handler,
}

impl Server {
    pub fn new(addr: String, handler: Handler) -> Self {
        Server { addr, handler }
    }

    pub fn listen_and_serve(&self) {
        // TODO: listen to tcp
        println!("listen and serve");
    }
}
