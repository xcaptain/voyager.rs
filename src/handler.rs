use http::response::Builder;
use http::{Request, Response};

pub type HandlerFunc = Box<dyn Fn(&mut Builder, &Request<()>) -> Response<String> + Sync + Send>;

pub struct Handler {
    f: HandlerFunc,
}

impl Handler {
    pub fn new(f: HandlerFunc) -> Self {
        Handler { f }
    }

    pub fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        (self.f)(w, r)
    }
}
