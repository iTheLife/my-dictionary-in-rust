extern crate hyper;
extern crate hyper_router;

use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::sync::Arc;

use hyper::{Body, Method, Request, Response, StatusCode};

use crate::state::State;

const KEY_NAME: &str = "key";
const VALUE_NAME: &str = "value";

pub async fn router_service(
    req: Request<Body>,
    state: Arc<State>,
) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let mut dictionary = state.dictionary.clone();

    match (method.clone(), path.as_str()) {
        (Method::GET, "/") => {
            let contents = fs::read_to_string("hello.html").unwrap();
            *response.body_mut() = Body::from(contents);
        }
        (Method::GET, "/get") => {
            let key = get_value_from_query(&req, KEY_NAME);
            let contents = dictionary.get(key.as_str());
            *response.body_mut() = Body::from(contents);
        }
        (Method::GET, "/all") => {
            let contents = dictionary.get_all();
            *response.body_mut() = Body::from(contents);
        }
        (Method::POST, "/remove") => {
            let key = get_value_from_query(&req, KEY_NAME);
            dictionary.remove(&key);
            *response.body_mut() = Body::from(format!("Значение по ключу: {} удалено", key));
        }
        (Method::POST, "/send") => {
            let mut key = String::new();
            let mut value = String::new();
            for (name, value_header) in req.headers() {
                match name.as_str() {
                    KEY_NAME => key = value_header.to_str().unwrap_or_default().to_string(),
                    VALUE_NAME => value = value_header.to_str().unwrap_or_default().to_string(),
                    _ => continue,
                }
            }

            println!("{}", key);
            println!("{}", value);
            dictionary.insert(key.to_string(), value.to_string());
            *response.body_mut() = Body::from("Well done!");
        }
        _ => {
            let contents = fs::read_to_string("404.html").unwrap();
            *response.body_mut() = Body::from(contents);
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    println!("{} {}", method, path);

    Ok(response)
}

fn get_value_from_query(req: &Request<Body>, key: &str) -> String {
    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);
    println!("{:?}", params.get(key));
    match params.get(key) {
        Some(value) => value.to_string(),
        None => return format!("Значение по ключу: {} не найдено", key),
    }
}
