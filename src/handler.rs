use http::response::Builder;
use http::{Request, Response};
use std::sync::Arc;

pub type HandlerFunc = Arc<Fn(&mut Builder, &Request<()>) -> Response<String> + Sync + Send>;

#[derive(Clone)]
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
