use toml;
use serde::Serialize;
use serde_derive::Deserialize;
use cached::proc_macro::once;
use std::fs;
use std::path::{Path, PathBuf};

#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct FileData {
    name: String,
    description: String,
    path: PathBuf,
}

#[once(time=43200)]  // Update the cache once every 12h
fn get_data() -> Vec<FileData> {
    // TODO: Make the path configurable in the config file
    let files_dir = Path::new("files");
    if !files_dir.is_dir() {
        panic!("The provided path is not a directory!");
    }

    // Vector that stores info from the toml files
    let mut files_arr: Vec<FileData> = vec![];

    // Iterate through the directory
    for entry in fs::read_dir(files_dir).expect("Cannot read the directory") {
        let entry = entry.expect("Cannot read the subdirectories");

        // Check if it's a dir
        if !entry.path().is_dir() {
            continue;
        }

        // Iterate through the subdirectories
        for subentry in fs::read_dir(entry.path()).expect("Cannot read the subdirectory") {
            let subentry = subentry.expect("Cannot read the contents of the subdirectory");

            if subentry.file_name() == "info.toml" {
                // Read the file's content
                let file_contents = match fs::read_to_string(subentry.path()) {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Error: file {:?} is unreadable", subentry.path());
                        continue;
                    },
                };

                // Parse the toml and push it to the Vector
                let mut item: FileData = match toml::from_str(&file_contents) {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Error: file {:?} is not properly formatted", subentry.path());
                        continue;
                    },
                };

                // Add full path to the file's name
                let item_path = item.path;
                item.path = entry.path().to_path_buf();
                item.path.push(item_path);

                // Push the CdnItem to the Vector
                files_arr.push(item);
            }
        }
    }

    // dbg!(&files_arr);
    files_arr
}

#[get("/")]
fn index() -> Template {
    let data = get_data();
    Template::render("index", context! {data: &data})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/files", FileServer::from(relative!["/files"]))
        .mount("/static", FileServer::from(relative!["/static"]))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
