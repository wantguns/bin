use rocket_dyn_templates::Template;

use std::collections::HashMap;
use std::path::Path;

use crate::models::paste_id::PasteId;
use crate::models::pretty::get_pretty_body;

#[get("/p/<id>", rank = 2)]
pub async fn pretty_retrieve(id: PasteId<'_>) -> Option<Template> {
    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    let contents = get_pretty_body(&filename, &String::from("txt"));

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("body", contents);
    let rendered = Template::render("pretty", &map);

    match tree_magic::match_filepath("text/plain", filepath) {
        true => Some(rendered),
        false => None,
    }
}
