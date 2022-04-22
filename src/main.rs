#[macro_use]
extern crate rocket;
use std::{fs, net::IpAddr, path::PathBuf};

use clap::Parser;
use once_cell::sync::Lazy;
use rocket::{
    figment::{providers::Env, Figment},
    shield::{NoSniff, Shield},
};
use rocket_dyn_templates::{tera::Tera, Template};
use rust_embed::RustEmbed;

mod models;
mod routes;

const BINARY_VERSION: &str =
    concat!(env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));
const SERVER_VERSION: &str = concat!(
    "bin v.",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("GIT_HASH"),
    ") (Rocket)"
);
static BINARY_ETAG: Lazy<String> =
    Lazy::new(|| sha256::digest(BINARY_VERSION));

#[derive(RustEmbed)]
#[folder = "templates/"]
struct EmbeddedTemplates;

fn setup_tera_engine(tera: &mut Tera) {
    // Register templates
    let base_html = EmbeddedTemplates::get("base.html.tera").unwrap();
    let index_html = EmbeddedTemplates::get("index.html.tera").unwrap();
    let pretty_html = EmbeddedTemplates::get("pretty.html.tera").unwrap();

    // and shove them in the tera instance
    tera.add_raw_templates(vec![
        ("base.html", std::str::from_utf8(&base_html.data).unwrap()),
        ("index.html", std::str::from_utf8(&index_html.data).unwrap()),
        (
            "pretty.html",
            std::str::from_utf8(&pretty_html.data).unwrap(),
        ),
    ])
    .expect("Could not add raw templates to the tera instance");
}

/// A minimal, opinionated pastebin
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    /// Path to the uploads folder
    #[clap(short, long, default_value = "./upload")]
    upload: std::path::PathBuf,

    /// Port on which the webserver runs
    #[clap(short, long, default_value_t = 6162)]
    port: u16,

    /// Address on which the webserver runs
    #[clap(short, long, default_value = "127.0.0.1")]
    address: IpAddr,

    /// Binary uploads file size limit (in MiB)
    #[clap(short, long, default_value_t = 100)]
    binary_upload_limit: i32,

    /// Include client description
    #[clap(short, long, env)]
    client_desc: bool,
}

pub fn get_parsed_args() -> Args {
    Args::parse()
}

pub fn get_upload_dir() -> PathBuf {
    get_parsed_args().upload
}

#[launch]
fn rocket() -> _ {
    let args = get_parsed_args();

    // Custom Fairings and Providers
    let shield = Shield::default().disable::<NoSniff>();
    let figment = Figment::from(rocket::Config::default())
        .merge(("port", &args.port))
        .merge(("address", &args.address))
        .merge(("template_dir", &args.upload)) // Required if embedding templates
        .merge(Env::prefixed("BIN_").global());

    // create the upload directory, if not already created
    fs::create_dir_all(&args.upload)
        .expect("Could not create the upload directory");

    rocket::custom(figment)
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
                routes::pretty_retrieve::pretty_retrieve_ext
            ],
        )
        .attach(shield)
        .attach(Template::custom(|engines| {
            setup_tera_engine(&mut engines.tera)
        }))
}
