use crate::http::handler::Handler;
use http::response::Builder;
use http::{Request, Response};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Mux {
    m: HashMap<String, MuxEntry>,
}

#[derive(Clone)]
struct MuxEntry {
    h: Handler,
    pattern: String,
}

impl Mux {
    pub fn new() -> Self {
        Mux { m: HashMap::new() }
    }

    /// regist router pattern
    pub fn handle(&mut self, pattern: String, handler: Handler) {
        let entry = MuxEntry {
            h: handler,
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// get handler from mux
    pub fn handler(&self, r: &Request<()>) -> Option<&Handler> {
        let path = r.uri().path().to_owned();
        if let Some(entry) = self.m.get(&path) {
            return Some(&entry.h);
        }
        None
    }

    pub fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
        // TODO: implement NotFoundHandler
        return self.serve_http_not_found(w, r);
    }

    pub fn serve_http_not_found(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        println!("handler: serve http 404 not found");
        let path = r.uri().path();
        w.body(format!("in 404 not found handler, path is: {}", path))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mux() {
        let mut mux = Mux::new();
        let handler = Handler::new();

        mux.handle("/hello".to_string(), handler);
        assert_eq!(1, 1);
    }
}
