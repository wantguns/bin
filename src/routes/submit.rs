use rocket::{form::Form, response::Redirect};

use std::fs;

use crate::models::paste_id::PasteId;

#[derive(FromForm)]
pub struct PasteIdForm {
    val: String,
    ext: String,
}

#[post("/submit", data = "<paste>")]
pub async fn submit(paste: Form<PasteIdForm>) -> Redirect {
    let id = PasteId::new(6);

    let filename = format!("upload/{id}", id = id);
    let content = &paste.val;
    let ext = &paste.ext;

    fs::write(&filename, content).expect("Unable to write to the file");

    Redirect::to(format!("/p/{id}.{ext}", id = id, ext = ext))
}
