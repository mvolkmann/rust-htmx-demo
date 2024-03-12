use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
            .service(healthcheck)
            // .service(hello)
            .route("/hello", web::get().to(hello))
            // .service(form)
            .route("/form", web::get().to(form))
            .service(rows)
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

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let message = "Everything is working fine".to_string();
    HttpResponse::Ok().json(message)
}

async fn hello(data: web::Data<AppState>) -> HttpResponse {
    println!("in hello");
    let mut context = Context::new();
    context.insert("name", "Tera");
    let html = data.templates.render("hello.tera", &context);
    println!("html = {:?}", html);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

// #[get("/form")]
async fn form(data: web::Data<AppState>) -> HttpResponse {
    let selected_dog_ref = &data.dog_map[&data.selected_id];
    let mut context = Context::new();
    context.insert("dog", selected_dog_ref);

    let html = data.templates.render("form.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}

#[get("/dog-rows")]
async fn rows(data: web::Data<AppState>) -> HttpResponse {
    let mut dogs = data.dog_map.values().collect::<Vec<&Dog>>();
    dogs.sort_by(|a, b| a.name.cmp(&b.name));
    let mut context = Context::new();
    context.insert("dogs", &dogs);

    let html = data.templates.render("dog-rows.tera", &context);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.unwrap())
}