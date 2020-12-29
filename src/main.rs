#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate tree_magic;

use std::io;
use std::path::Path;
use std::fs::File;

use rocket::Data;
use rocket::http::RawStr;
use rocket_contrib::templates::Template;

mod paste_id;

use paste_id::PasteId;

#[get("/p/<id>")]
fn pretty_retrieve(id: PasteId) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    let filepath = Path::new(&filename);

    match tree_magic::from_filepath(filepath).as_str() {
        "text/plain" => File::open(&filename).ok(),
            _   =>    Err("does not have the MIME type of a plaintext file").ok()
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
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/")]
fn index() -> &'static str {
    "\
USAGE
=====

    POST    / 

        accepts raw data in the body of the request and responds with a URL
        of a page containing the body's content

    GET     /<id>

        retrieves the content for the paste with id `<id>`

    GET     /p/<id>

        retrieves the content for the paste with id `<id>`, with syntax highlighting
    "
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve, pretty_retrieve])
        .launch();
}
