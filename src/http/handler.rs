#[derive(Clone)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
    pub fn serve_http(&self) {
        println!("handler: serve http");
    }
}
