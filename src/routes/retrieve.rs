use std::fs::File;

use crate::models::paste_id::PasteId;

#[get("/<id>")]
pub async fn retrieve(id: PasteId<'_>) -> Option<File> {
    let filename = format!("upload/{id}", id = id);

    File::open(&filename).ok()
}