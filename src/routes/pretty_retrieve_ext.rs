use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use std::collections::HashMap;
use std::io::ErrorKind::InvalidData;
use std::path::Path;

use crate::get_upload_dir;
use crate::models::maybe_redirect::MaybeRedirect;
use crate::models::pretty::get_pretty_body;
use crate::models::pretty_syntax::PasteIdSyntax;

#[get("/p/<id_ext>", rank = 1)]
pub async fn pretty_retrieve_ext(
    id_ext: PasteIdSyntax<'_>,
) -> Option<MaybeRedirect> {
    let id = id_ext.get_fname();
    let ext = id_ext.get_ext();

    let filepath = Path::new(&get_upload_dir()).join(id.to_string());

    let contents = match get_pretty_body(&filepath, &ext.to_string()) {
        Ok(v) => v,
        Err(e) if e.kind() == InvalidData => {
            return Some(Redirect::to(format!("/{}", id)).into());
        }
        _ => {
            return None;
        }
    };

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("body", contents);
    let rendered = Template::render("pretty.html", &map);

    match tree_magic::match_filepath("text/plain", &filepath) {
        true => Some(rendered.into()),
        false => None,
    }
}
