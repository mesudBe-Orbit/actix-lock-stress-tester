use actix_web::{get, web, App, HttpServer};
use actix_web::web::Data;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
struct State {
    pub values: HashMap<String, String>,
    pub values_mutex: Arc<Mutex<HashMap<String, String>>>,
}

impl State {
    pub fn new() -> Self {
        let values = vec![(String::from("key"), String::from("value"))]
            .into_iter()
            .collect::<HashMap<String, String>>();
        Self {
            values_mutex: Arc::new(Mutex::new(values.clone())),
            values,
        }
    }
}

#[get("/lock")]
async fn get_lock(state: Data<State>) -> String {
    let vals = state.values_mutex.lock().unwrap();
    vals.get("key").unwrap().clone()
}

#[get("/nolock")]
async fn get_nolock(state: Data<State>) -> String {
    state.values.get("key").unwrap().clone()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_lock)
            .service(get_nolock)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}