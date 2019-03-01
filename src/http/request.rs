#[derive(Clone, Default)]
pub struct Request {
    pub method: String,
    pub url: Url,
    pub proto: String,
}

impl Request {
    pub fn new(method: String, path: String, proto: String) -> Self {
        Request {
            method,
            url: Url::new(path),
            proto,
        }
    }
}

#[derive(Clone, Default)]
pub struct Url {
    pub path: String,
}

impl Url {
    pub fn new(path: String) -> Self {
        Url { path }
    }
}
