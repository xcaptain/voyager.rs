#[derive(Default)]
pub struct ResponseWriter {}

impl ResponseWriter {
    pub fn new() -> Self {
        ResponseWriter {}
    }

    pub fn write(&self, body: String) {
        println!("write from response writer: {}", body);
    }
}
