use std::fs::File;

use crate::get_upload_dir;
use crate::models::paste_id::PasteId;
use crate::models::pretty_syntax::PasteIdSyntax;

#[get("/<id>", rank = 2)]
pub async fn retrieve(id: PasteId<'_>) -> Option<File> {
    // let filename = format!("upload/{id}", id = id);

    File::open(get_upload_dir().join(format!("{id}", id = id))).ok()
}

// rank 1 here because this would be more oftenly used
#[get("/<id_ext>", rank = 1)]
pub async fn retrieve_ext(id_ext: PasteIdSyntax<'_>) -> Option<File> {
    // let filename = format!("upload/{id}", id = id_ext.get_fname());

    File::open(get_upload_dir().join(format!("{id}", id = id_ext.get_fname())))
        .ok()
}
