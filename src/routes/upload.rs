use rocket::data::{Data, ToByteUnit};

use std::path::Path;

use crate::models::paste_id::PasteId;

#[post("/", data = "<paste>")]
pub async fn upload(paste: Data<'_>) -> Result<String, std::io::Error> {
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

    Ok(url)
}
