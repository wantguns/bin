use rocket::{form::Form, response::Redirect};

use std::{fs, path::Path};

use crate::get_upload_dir;
use crate::models::paste_id::PasteId;

#[derive(FromForm)]
pub struct PasteIdForm {
    content: String,
    ext: String,
}

#[post("/submit", data = "<paste>")]
pub async fn submit(paste: Form<PasteIdForm>) -> Redirect {
    let id = PasteId::new(6);

    let filepath = Path::new(&get_upload_dir()).join(format!("{id}", id = id));
    let content = &paste.content;
    let ext = &paste.ext;

    fs::write(filepath, content).expect("Unable to write to the file");

    Redirect::to(format!("/p/{id}.{ext}", id = id, ext = ext))
}
