use http::response::Builder;
use http::{Request, Response};
use std::sync::Arc;

#[derive(Clone)]
pub struct Handler {
    f: Arc<Fn(&mut Builder, &Request<()>) -> Response<String> + Sync + Send>,
}

impl Handler {
    pub fn new(f: Arc<Fn(&mut Builder, &Request<()>) -> Response<String> + Sync + Send>) -> Self {
        Handler { f }
    }

    pub fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        println!("handler: serve http");
        (self.f)(w, r)
    }
}
