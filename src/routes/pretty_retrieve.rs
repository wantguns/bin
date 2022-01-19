use rocket_dyn_templates::Template;

use std::collections::HashMap;
use std::path::Path;

use crate::get_upload_dir;
use crate::models::paste_id::PasteId;
use crate::models::pretty::get_pretty_body;

#[get("/p/<id>", rank = 2)]
pub async fn pretty_retrieve(id: PasteId<'_>) -> Option<Template> {
    let filepath = Path::new(&get_upload_dir()).join(format!("{id}", id = id));

    let contents = get_pretty_body(&filepath, &String::from("txt"));

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("body", contents);
    let rendered = Template::render("pretty.html", &map);

    match tree_magic::match_filepath("text/plain", &filepath) {
        true => Some(rendered),
        false => None,
    }
}
