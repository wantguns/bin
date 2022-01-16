use rocket_dyn_templates::Template;

use std::collections::HashMap;

#[get("/")]
pub async fn index() -> Option<Template> {
    let mut map = HashMap::new();
    map.insert("title", "bin");
    Some(Template::render("index", &map))
}
