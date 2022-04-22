use rocket_dyn_templates::Template;

use std::collections::HashMap;

use crate::get_parsed_args;
use crate::models::response_wrapper::ResponseWrapper;

#[get("/")]
pub async fn index() -> ResponseWrapper<Template> {
    let mut map = HashMap::new();

    // whether to include `/client` info
    let client_desc = match get_parsed_args().client_desc {
        true => "placeholder",
        false => "",
    };

    map.insert("title", "bin");
    map.insert("client_desc", client_desc);

    ResponseWrapper::meta_response(Template::render("index.html", &map))
}
