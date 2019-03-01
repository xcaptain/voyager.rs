use crate::http::request::Request;
use crate::http::response::ResponseWriter;

#[derive(Clone, Default)]
pub struct Handler<T>
where
    T: Fn(&ResponseWriter, &Request) + Clone,
{
    f: T,
}

impl<T> Handler<T>
where
    T: Fn(&ResponseWriter, &Request) + Clone,
{
    pub fn new(f: T) -> Self {
        Handler { f }
    }

    pub fn serve_http(&self, w: &ResponseWriter, r: &Request) {
        println!("handler: serve http");
        (self.f)(w, r)
    }
}
