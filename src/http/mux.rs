use crate::http::handler::Handler;
use std::collections::HashMap;

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

    // regist router pattern
    pub fn handle(&mut self, pattern: String, handler: Handler) {
        let entry = MuxEntry {
            h: handler,
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    // get handler from mux
    pub fn handler(&self, pattern: String) -> Option<Handler> {
        if let Some(entry) = self.m.get(&pattern) {
            return Some(entry.clone().h);
        }
        return None;
    }

    pub fn serve_http(&self) {
        println!("mux.rs: serve http");
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
        let _saved_handler = mux.handler("/hello".to_string());

        // TODO: compare 2 handler is the same
        assert_eq!(1, 1);
    }
}
