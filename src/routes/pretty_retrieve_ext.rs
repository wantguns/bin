use rocket_dyn_templates::Template;

use std::collections::HashMap;
use std::path::Path;

use crate::models::pretty_syntax::PasteIdSyntax;
use crate::models::pretty::get_pretty_body;

#[get("/p/<id_ext>", rank = 1)]
pub async fn pretty_retrieve_ext(id_ext: PasteIdSyntax<'_>) -> Option<Template> {
    let id = id_ext.get_fname();
    let ext = id_ext.get_ext();

    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    let contents = get_pretty_body(&filename, &ext.to_string());

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("body", contents);
    let rendered = Template::render("pretty", &map);

    match tree_magic::match_filepath("text/plain", filepath) {
        true => Some(rendered),
        false => None,
    }
}
