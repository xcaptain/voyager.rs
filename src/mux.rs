use crate::handler::{Handler, HandlerFunc};
use http::response::Builder;
use http::{Request, Response, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Mux {
    m: HashMap<String, MuxEntry>,
    not_found_handler: Handler,
}

#[derive(Clone)]
struct MuxEntry {
    h: Handler,
    pattern: String,
}

impl Mux {
    pub fn new() -> Self {
        let default_not_found_handler = Handler::new(Arc::new(
            |w: &mut Builder, r: &Request<()>| -> Response<String> {
                let path = r.uri().path();
                w.status(StatusCode::NOT_FOUND)
                    .body(format!("404 not found, path is: {}", path))
                    .unwrap()
            },
        ));
        Mux {
            m: HashMap::new(),
            not_found_handler: default_not_found_handler,
        }
    }

    /// register router pattern
    pub fn handle(&mut self, pattern: String, handler: Handler) {
        let entry = MuxEntry {
            h: handler,
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// just a piece of syntax sugar of `handle`
    pub fn handle_func(&mut self, pattern: String, handler: HandlerFunc) {
        let entry = MuxEntry {
            h: Handler::new(handler),
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// register custom not found handler
    pub fn handle_not_found(&mut self, handler: Handler) {
        self.not_found_handler = handler;
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
        // match router
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
        self.not_found_handler.serve_http(w, r)
    }
}

impl Default for Mux {
    fn default() -> Self {
        Self::new()
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
