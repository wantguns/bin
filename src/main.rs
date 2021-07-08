#[macro_use]
extern crate rocket;
use rocket::data::{Data, ToByteUnit};
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::Template;

extern crate tree_magic;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

mod paste_id;
mod pretty;
mod pretty_syntax;

use paste_id::PasteId;
use pretty::get_pretty_body;
use pretty_syntax::PasteIdSyntax;

#[get("/p/<id_ext>", rank = 1)]
async fn pretty_retrieve_ext(id_ext: PasteIdSyntax<'_>) -> Option<Template> {
    let id = id_ext.get_fname();
    let ext = id_ext.get_ext();

    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    let contents = get_pretty_body(&filename, &ext.to_string());
    let theme = env::var("THEME").unwrap_or("".to_string());

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("theme", theme);
    map.insert("body", contents);
    let rendered = Template::render("pretty", &map);

    match tree_magic::match_filepath("text/plain", filepath) {
        true => Some(rendered),
        false => None,
    }
}

#[get("/p/<id>", rank = 2)]
async fn pretty_retrieve(id: PasteId<'_>) -> Option<Template> {
    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    let contents = get_pretty_body(&filename, &String::from("txt"));
    let theme = env::var("THEME").unwrap_or("".to_string());

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("theme", theme);
    map.insert("body", contents);
    let rendered = Template::render("pretty", &map);

    match tree_magic::match_filepath("text/plain", filepath) {
        true => Some(rendered),
        false => None,
    }
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    let filename = format!("upload/{id}", id = id);

    File::open(&filename).ok()
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<Redirect, std::io::Error> {
    let id = PasteId::new(6);

    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    paste.open(100.mebibytes()).into_file(filepath).await?;

    let url = match tree_magic::from_filepath(filepath)
        .as_str()
        .contains("text")
    {
        true => format!("/p/{id}", id = id),

        false => format!("/{id}", id = id),
    };

    Ok(Redirect::to(url))
}

#[derive(FromForm)]
struct PasteIdForm {
    val: String,
}

#[post("/submit", data = "<paste>")]
async fn submit(paste: Form<PasteIdForm>) -> Redirect {
    let id = PasteId::new(4);

    let filename = format!("upload/{id}", id = id);
    let content = paste.into_inner().val;

    fs::write(&filename, content).expect("Unable to write to the file");

    Redirect::to(format!("/p/{id}", id = id))
}

#[get("/")]
async fn index() -> Option<Template> {
    let mut map = HashMap::new();
    map.insert("title", "bin");
    Some(Template::render("index", &map))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                upload,
                submit,
                retrieve,
                pretty_retrieve,
                pretty_retrieve_ext
            ],
        )
        .attach(Template::fairing())
}
