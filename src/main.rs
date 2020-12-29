#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate tree_magic;

use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

use rocket_contrib::templates::Template;
use rocket::Data;

mod paste_id;

use paste_id::PasteId;

#[get("/p/<id>")]
fn pretty_retrieve(id: PasteId) -> Option<Template> {
    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);
    let mut file = File::open(&filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let theme = String::from(".");

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("theme", theme);
    map.insert("code", contents);
    let rendered = Template::render("pretty", &map);

    match tree_magic::from_filepath(filepath).contains("text") {
        true    =>  Some(rendered),
        false   =>  None
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

    let url = match tree_magic::from_filepath(filepath).as_str().contains("text") {
        true    => format!("{host}/p/{id}\n", host = "http://localhost:8000", id = id),
        false   => format!("{host}/{id}\n", host = "http://localhost:8000", id = id),
    };

    Ok(url)
}

#[get("/")]
fn index() -> &'static str {
    "\
    USAGE
    -----

        POST    / 

            accepts raw data in the body of the request and responds with a URL
            of a page containing the body's content

        GET     /<id>

            retrieves the content for the paste with id `<id>`

        GET     /p/<id>

            retrieves the HTML page with syntax-highlighted content for the paste with id `<id>`
 
    EXAMPLES
    --------

        Paste a file named 'file.txt' using cURL:

            curl -d@file.txt https://bin.wantguns.dev

        Paste from stdin using cURL:

            echo \"Hello, world.\" | curl -d@- https://bin.wantguns.dev

        Add this to your .zshrc to implement a quicker usage.

            function paste() {
              local file=${1:-/dev/stdin}
              curl -d@${file} https://bin.wantguns.dev
            }

        If the uploaded data binary is parsed as \"text/*\", then the paste will be syntax
        highlighted
    "
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve, pretty_retrieve])
        .attach(Template::fairing())
        .launch();
}
