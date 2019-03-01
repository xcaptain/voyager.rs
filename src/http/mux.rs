use crate::http::handler::Handler;
use crate::http::request::Request;
use crate::http::response::ResponseWriter;
use std::collections::HashMap;

#[derive(Default)]
pub struct Mux<T>
where
    T: Fn(&ResponseWriter, &Request) + Clone,
{
    m: HashMap<String, MuxEntry<T>>,
}

#[derive(Clone)]
struct MuxEntry<T>
where
    T: Fn(&ResponseWriter, &Request) + Clone,
{
    h: Handler<T>,
    pattern: String,
}

impl<T> Mux<T>
where
    T: Fn(&ResponseWriter, &Request) + Clone,
{
    pub fn new() -> Self {
        Mux { m: HashMap::new() }
    }

    /// regist router pattern
    pub fn handle(&mut self, pattern: String, handler: Handler<T>) {
        let entry = MuxEntry {
            h: handler,
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// get handler from mux
    pub fn handler(&self, r: &Request) -> Option<Handler<T>> {
        let path = r.url.path.clone();
        if let Some(entry) = self.m.get(&path) {
            return Some(entry.clone().h.clone());
        }
        None
    }

    pub fn serve_http(&self, w: &ResponseWriter, r: &Request) {
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
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
