#[macro_use]
extern crate rocket;
use std::{fs, net::IpAddr, path::PathBuf};

use clap::Parser;
use rocket::shield::{NoSniff, Shield};
use rocket_dyn_templates::Template;

mod models;
mod routes;

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
    #[clap(short, long, default_value = "0.0.0.0")]
    address: IpAddr,

    /// Binary uploads file size limit (in MiB)
    #[clap(short, long, default_value_t = 100)]
    binary_upload_limit: i32,
}

pub fn get_parsed_args() -> Args {
    Args::parse()
}

pub fn get_upload_dir() -> PathBuf {
    get_parsed_args().upload
}

#[launch]
fn rocket() -> _ {
    let shield = Shield::default().disable::<NoSniff>();
    let args = get_parsed_args();

    // create the upload directory, if not already created
    fs::create_dir_all(args.upload)
        .expect("Could not create the upload directory");

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
