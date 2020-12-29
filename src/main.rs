#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::io;
use std::path::Path;
use std::fs::File;

use rocket::Data;
use rocket::http::RawStr;

mod paste_id;

use paste_id::PasteId;

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
    "
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve])
        .launch();
}
