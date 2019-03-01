use crate::http::request::Request;
use crate::http::response::ResponseWriter;

#[derive(Clone, Default)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }

    pub fn serve_http(&self, _w: &ResponseWriter, _r: &Request) {
        println!("handler: serve http");
    }
}
