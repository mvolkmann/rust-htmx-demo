use tera::Tera; // templating engine based on Jinja

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
struct Dog {
    id: String,
    name: String,
    breed: String,
}

#[derive(Clone, Copy)]
struct AppState {
  dog_map: HashMap<String, Dog>,
  selected_id: String,
  templates: tera::Tera
}

fn main() {
}