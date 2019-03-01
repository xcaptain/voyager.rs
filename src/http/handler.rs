use crate::http::request::Request;
use crate::http::response::ResponseWriter;

pub struct Handler {
    f: Box<Fn(&ResponseWriter, &Request)>,
}

impl Handler {
    pub fn new(f: Box<Fn(&ResponseWriter, &Request)>) -> Self {
        Handler { f }
    }

    pub fn serve_http(&self, w: &ResponseWriter, r: &Request) {
        println!("handler: serve http");
        (self.f)(w, r)
    }
}
