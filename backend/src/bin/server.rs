use serde::{Deserialize, Serialize};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let db = Arc::new(Mutex::new(vec![Todo {
        id: 0,
        text: String::from("hello"),
        completed: false,
    }]));

    let index = warp::path!("todos").map({
        let db = db.clone();
        move || format!("{db:?}")
    });

    let add = warp::path!("todos" / "add" / u64).and_then({
        let db = db.clone();
        move |id| {
            let db = db.clone();
            async move {
                db.lock().unwrap().push(Todo {
                    id,
                    text: String::from("new todo"),
                    completed: false,
                });

                Ok::<_, Infallible>(format!("{db:?}"))
            }
        }
    });

    warp::serve(index.or(add)).run(([127, 0, 0, 1], 3030)).await;
}
