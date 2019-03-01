use crate::http::handler::Handler;
use crate::http::request::Request;
use crate::http::response::ResponseWriter;
use std::collections::HashMap;

#[derive(Default)]
pub struct Mux {
    m: HashMap<String, MuxEntry>,
}

// #[derive(Clone)]
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
    pub fn handler(&self, r: &Request) -> Option<&Handler> {
        let path = r.url.path.clone();
        if let Some(entry) = self.m.get(&path) {
            return Some(&entry.h);
        }
        None
    }

    pub fn serve_http(&self, w: &ResponseWriter, r: &Request) {
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
        // TODO: implement NotFoundHandler
        println!("404 not found");
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
