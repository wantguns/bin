#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate tree_magic;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::Data;
use rocket_contrib::templates::Template;

mod paste_id;

use paste_id::PasteId;

#[get("/p/<id>")]
fn pretty_retrieve(id: PasteId) -> Option<Template> {
    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);
    let mut file = File::open(&filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let theme = env::var("THEME").unwrap_or("".to_string());

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("theme", theme);
    map.insert("code", contents);
    let rendered = Template::render("pretty", &map);

    match tree_magic::match_filepath("text/plain", filepath) {
        true => Some(rendered),
        false => None,
    }
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File> {
    let filename = format!("upload/{id}", id = id);

    File::open(&filename).ok()
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = PasteId::new(4);

    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    //TODO: Implement limits when this stupid framework starts working
    //      Look into using open(), take() methods in the Data struct
    paste.stream_to_file(filepath)?;

    let url = match tree_magic::from_filepath(filepath)
        .as_str()
        .contains("text")
    {
        true => format!(
            "https://{host}/p/{id}\n",
            host = env::var("HOST_URL").unwrap_or("<no_host_provided>".to_string()),
            id = id
        ),

        false => format!(
            "https://{host}/{id}\n",
            host = env::var("HOST_URL").unwrap_or("http://localhost:8000".to_string()),
            id = id
        ),
    };

    Ok(url)
}

#[derive(FromForm)]
struct PasteIdForm {
    val: String,
}

#[post("/submit", data = "<paste>")]
fn submit(paste: Form<PasteIdForm>) -> Redirect {
    let id = PasteId::new(4);

    let filename = format!("upload/{id}", id = id);
    let content = paste.into_inner().val;

    fs::write(&filename, content).expect("Unable to write to the file");

    Redirect::to(format!("/p/{id}", id = id))
}

#[get("/")]
fn index() -> Option<Template> {
    let mut map = HashMap::new();
    map.insert("title", "bin");
    Some(Template::render("index", &map))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, upload, submit, retrieve, pretty_retrieve],
        )
        .attach(Template::fairing())
        .launch();
}
