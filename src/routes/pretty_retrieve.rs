use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::fs;

use std::collections::HashMap;
use std::io::ErrorKind::{InvalidData, NotFound};
use std::path::Path;

use crate::get_upload_dir;
use crate::models::paste_id::PasteId;
use crate::models::pretty::get_pretty_body;
use crate::models::pretty_syntax::PasteIdSyntax;
use crate::models::response_wrapper::ResponseWrapper;

#[get("/p/<id>", rank = 2)]
pub async fn pretty_retrieve(id: PasteId<'_>) -> ResponseWrapper<Template> {
    pretty_retrieve_inner(&id.to_string(), "txt").await
}

#[get("/p/<id_ext>", rank = 1)]
pub async fn pretty_retrieve_ext(
    id_ext: PasteIdSyntax<'_>,
) -> ResponseWrapper<Template> {
    let id = id_ext.get_fname();
    let ext = id_ext.get_ext();

    pretty_retrieve_inner(id, ext).await
}

pub async fn pretty_retrieve_inner(
    id: &str,
    ext: &str,
) -> ResponseWrapper<Template> {
    let filepath = Path::new(&get_upload_dir()).join(id);

    let modified_date =
        match fs::metadata(&filepath).and_then(|m| m.modified()) {
            Ok(v) => v,
            Err(e) if e.kind() == NotFound => {
                return ResponseWrapper::not_found(id);
            }
            Err(e) => {
                return ResponseWrapper::server_error(e.to_string());
            }
        };

    let contents = match get_pretty_body(&filepath, ext) {
        Ok(v) => v,
        Err(e) if e.kind() == InvalidData => {
            return ResponseWrapper::redirect(Redirect::permanent(format!(
                "/{}",
                id
            )));
        }
        Err(e) if e.kind() == NotFound => {
            return ResponseWrapper::not_found(id)
        }
        Err(e) => {
            return ResponseWrapper::server_error(e.to_string());
        }
    };

    let mut map = HashMap::new();
    map.insert("title", id.to_string());
    map.insert("body", contents);
    let rendered = Template::render("pretty.html", &map);

    match tree_magic::match_filepath("text/plain", &filepath) {
        true => {
            ResponseWrapper::pretty_paste_response(rendered, modified_date)
        }
        false => ResponseWrapper::server_error("media type unacceptable"),
    }
}
