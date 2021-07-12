use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/<file..>", rank = 3)]
pub async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
