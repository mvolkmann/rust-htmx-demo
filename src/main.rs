use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::{Context, Tera}; // templating engine based on Jinja
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Dog {
    id: String,
    name: String,
    breed: String,
}

struct AppState {
  dog_map: HashMap<String, Dog>,
  selected_id: String,
  templates: tera::Tera
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
        println!("creating server");
        let templates = Tera::new("src/templates/**/*.tera").unwrap();

        let mut state = AppState {
            dog_map: HashMap::new(),
            selected_id: String::new(),
            templates,
        };

        add_dog(&mut state.dog_map, "Comet", "Whippet");
        add_dog(&mut state.dog_map, "Oscar", "German Shorthaired Pointer");
        println!("dog_map = {:?}", state.dog_map);
 
        App::new()
            .app_data(web::Data::new(state))
            .route("/hello", web::get().to(hello))
            .route("/dogs", web::get().to(dogs))
            .route("/form", web::get().to(form))
            .route("/rows", web::get().to(rows))
            .service(Files::new("/", "./public").index_file("index.html"))
    })
    .workers(1) 
    .bind("127.0.0.1:3000")
    .unwrap()
    .run();

    // println!("listening at {}", server.local_addr());
    println!("listening at 3000");
    server.await
}

async fn dogs(data: web::Data<AppState>) -> HttpResponse {
    println!("data.selected_id = {:?}", data.selected_id);

    let mut context = Context::new();
    context.insert("name", "Tera");

    let mut dogs = data.dog_map.values().collect::<Vec<&Dog>>();
    dogs.sort_by(|a, b| a.name.cmp(&b.name));
    context.insert("dogs", &dogs);

    let html = data.templates.render("dogs.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

async fn form(data: web::Data<AppState>) -> HttpResponse {
    let mut context = Context::new();
    let id = &data.selected_id;
    if !id.is_empty() {
      let dog_ref = &data.dog_map[&data.selected_id];
      context.insert("dog", dog_ref);
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
    let mut dogs = data.dog_map.values().collect::<Vec<&Dog>>();
    dogs.sort_by(|a, b| a.name.cmp(&b.name));
    context.insert("dogs", &dogs);

    let html = data.templates.render("dog-rows.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}