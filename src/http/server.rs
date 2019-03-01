use crate::http::mux::Mux;
use crate::http::request::Request;
use crate::http::response::ResponseWriter;

pub struct Server {
    pub addr: String,
    pub m: Mux,
}

impl Server {
    pub fn new(addr: String, m: Mux) -> Self {
        Server { addr, m }
    }

    /// let mux to handle serving logic stuff
    pub fn listen_and_serve(&self) {
        // TODO: setup request and response, listen to tcp
        // request should be created from tcp stream using tokio
        println!("listen and serve");
        let w = ResponseWriter::new();
        let r = Request::new(
            "GET".to_string(),
            "/world".to_string(),
            "http 1.0".to_string(),
        );
        self.m.serve_http(&w, &r)
    }
}
