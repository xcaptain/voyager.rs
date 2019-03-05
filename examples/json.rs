use http::response::Builder;
use http::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use voyager::http as myhttp;
use voyager::mux::{DefaultHandler, DefaultMux, HandlerFunc};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut m = DefaultMux::new();

    // this object must be shared across threads, so must be wrapped
    // by Arc to keep thread safe
    let persons: Arc<Vec<Person>> = Arc::new(vec![
        Person {
            id: 1,
            name: "person1".to_string(),
            age: 10,
        },
        Person {
            id: 2,
            name: "person2".to_string(),
            age: 20,
        },
    ]);
    m.handle(
        "/person".to_string(),
        DefaultHandler::new(find_person(persons.clone())),
    );
    m.handle(
        "/persons".to_string(),
        DefaultHandler::new(get_persons(persons.clone())),
    );

    return myhttp::listen_and_serve("127.0.0.1:8080".to_string(), m);
}

#[derive(Serialize, Deserialize)]
struct Person {
    id: usize,
    name: String,
    age: u8,
}

fn find_person(persons: Arc<Vec<Person>>) -> HandlerFunc {
    let handler = move |w: &mut Builder, _r: &Request<()>| -> Response<String> {
        let person = &persons[0];
        match serde_json::to_string(&person) {
            Ok(body) => w
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
            Err(e) => w
                .status(StatusCode::BAD_REQUEST)
                .body(format!("serialize failed, {}", e))
                .unwrap(),
        }
    };
    Box::new(handler)
}

fn get_persons(persons: Arc<Vec<Person>>) -> HandlerFunc {
    let handler = move |w: &mut Builder, _r: &Request<()>| -> Response<String> {
        match serde_json::to_string(&persons) {
            Ok(body) => w
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
            Err(e) => w
                .status(StatusCode::BAD_REQUEST)
                .body(format!("serialize failed, {}", e))
                .unwrap(),
        }
    };
    Box::new(handler)
}
