mod config;
mod data;

use crate::config::get_config;
use crate::data::get_data;

#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};
use rocket::Request;
use rocket::http::Status;

#[get("/")]
fn index() -> Template {
    let data = get_data();
    let config = get_config();
    Template::render("index", context! {
        data:   &data,
        config: &config,
    })
}

#[catch(404)]
fn not_found(status: Status, _req: &Request) -> Template {
    let config = get_config();
    let code = status.code;

    Template::render("error", context! {
        code:    code,
        title:   format!("{}: Not Found", code),
        message: "The requested page does not exist",
        config: &config,
    })
}

#[launch]
fn rocket() -> _ {
    let config = get_config();

    // Check if the files dir exists
    if !config.files_path.is_dir() {
        eprintln!("The provided path {:?} does not exist, or is unreadable", config.files_path);
        std::process::exit(1);
    }

    let figment = rocket::Config::figment()
        .merge(("port", config.http_port));

    println!("The server has started! Visit it at http://127.0.0.1:{}", config.http_port);

    rocket::custom(figment)
        .mount("/files", FileServer::from(&config.files_path))
        .mount("/static", FileServer::from(relative!["/static"]))
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
