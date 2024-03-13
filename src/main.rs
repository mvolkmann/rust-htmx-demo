use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera}; // templating engine based on Jinja
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Dog {
    id: String,
    name: String,
    breed: String,
}

#[derive(Debug)]
struct AppState {
    dog_map: Mutex<HashMap<String, Dog>>,
    selected_id: Mutex<String>,
    templates: tera::Tera,
}

fn add_dog(dog_map: &mut HashMap<String, Dog>, name: &str, breed: &str) -> Dog {
    let uuid = Uuid::new_v4();
    let dog = Dog {
        id: String::from(uuid),
        name: name.to_string(),
        breed: breed.to_string(),
    };
    dog_map.insert(String::from(uuid), dog.clone());
    dog
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("in main");
    let server = HttpServer::new(|| {
        let state = Arc::new(Mutex::new(AppState {
            dog_map: Mutex::new(HashMap::new()),
            selected_id: Mutex::new(String::new()),
            templates: Tera::new("src/templates/**/*.tera").unwrap(),
        }));

        {
            // New scope to extend the lifetime of the lock
            let state_lock = state.lock().unwrap();
            let mut dog_map = state_lock.dog_map.lock().unwrap();
            add_dog(&mut dog_map, "Comet", "Whippet");
            add_dog(&mut dog_map, "Oscar", "German Shorthaired Pointer");
            println!("dog_map = {:?}", *dog_map);
            // state_lock is dropped here, releasing the lock
        }

        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/hello", web::get().to(hello))
            .route("/dogs", web::get().to(dogs))
            .route("/form", web::get().to(form))
            .route("/rows", web::get().to(rows))
            .route("/select/{id}", web::get().to(select))
            .service(Files::new("/", "./public").index_file("index.html"))
    })
    .workers(1)
    .bind("127.0.0.1:3000")
    .unwrap()
    .run()
    .map_err(|e| {
        eprintln!("Error starting server: {:?}", e);
        e
    });

    // println!("listening at {}", server.local_addr());
    println!("listening on 3000");
    server.await
}

async fn dogs(data: web::Data<AppState>) -> HttpResponse {
    println!("data.selected_id = {:?}", data.selected_id);

    let mut context = Context::new();
    context.insert("name", "Tera");

    let dog_map = data.dog_map.lock().unwrap();
    let mut dogs = dog_map.values().collect::<Vec<&Dog>>();
    dogs.sort_by(|a, b| a.name.cmp(&b.name));
    context.insert("dogs", &dogs);

    let html = data.templates.render("dogs.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

async fn form(data: web::Data<AppState>) -> HttpResponse {
    let id = data.selected_id.lock().unwrap();
    println!("form: id = {:?}", id);
    // let mut context = Context::new();
    let context = Context::new();
    if !id.is_empty() {
        println!("form: id is not empty");
        // let dog_ref = &data.dog_map[&id];
        // context.insert("dog", dog_ref);
    }

    let html = data.templates.render("form.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

async fn hello(data: web::Data<AppState>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("name", "Tera");
    let html = data.templates.render("hello.tera", &context);
    println!("html = {:?}", html);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

async fn rows(data: web::Data<AppState>) -> HttpResponse {
    let mut context = Context::new();
    let dog_map = data.dog_map.lock().unwrap();
    let mut dogs = dog_map.values().collect::<Vec<&Dog>>();
    dogs.sort_by(|a, b| a.name.cmp(&b.name));
    context.insert("dogs", &dogs);

    let html = data.templates.render("dog-rows.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

// async fn select(data: web::Data<AppState>, info: web::Path<(String,)>) -> HttpResponse {
// The type for Path is a tuple that lists the types of the path parameters.
async fn select(params: web::Path<String>, data: web::Data<Arc<AppState>>) -> HttpResponse {
    let id = params.into_inner();
    println!("id = {:?}", id);
    println!("data = {:?}", data);
    let mut selected_id = data.selected_id.lock().unwrap();
    *selected_id = id;

    HttpResponse::Ok()
        .insert_header(("HX-Trigger", "selection-change"))
        .body("")
}
