use crate::handler::{Handler, HandlerFunc};
use http::response::Builder;
use http::{Request, Response, StatusCode};
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

    /// register router pattern
    pub fn handle(&mut self, pattern: String, handler: Handler) {
        let entry = MuxEntry {
            h: handler,
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    pub fn handle_func(&mut self, pattern: String, handler: HandlerFunc) {
        let entry = MuxEntry {
            h: Handler::new(handler),
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
        self.serve_http_not_found(w, r)
    }

    pub fn serve_http_not_found(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        let path = r.uri().path();
        w.status(StatusCode::NOT_FOUND)
            .body(format!("in 404 not found handler, path is: {}", path))
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
