#[macro_use]
extern crate rocket;
use rocket::shield::{NoSniff, Shield};
use rocket_dyn_templates::Template;

mod models;
mod routes;

#[launch]
fn rocket() -> _ {
    let shield = Shield::default().disable::<NoSniff>();

    rocket::build()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::static_files::static_files,
                routes::upload::upload,
                routes::submit::submit,
                routes::retrieve::retrieve,
                routes::retrieve::retrieve_ext,
                routes::pretty_retrieve::pretty_retrieve,
                routes::pretty_retrieve_ext::pretty_retrieve_ext
            ],
        )
        .attach(shield)
        .attach(Template::fairing())
}
