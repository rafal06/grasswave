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
    tags: Vec<String>,
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    displayed_name: String,
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

#[once]
fn get_config() -> Config {
    let config_file = Path::new("config.toml");

    if config_file.is_file() {
        // Read, parse and return it
        let config_file_contents = fs::read_to_string(config_file).unwrap();
        match toml::from_str(&config_file_contents) {
            Ok(val) => val,
            Err(_) => {
                //  Use default values instead (but don't save them)
                eprintln!("Error: config file is not properly formatted");
                gen_default_config(false)
            },
        }
    } else {
        // Save and return a default config
        println!("No config file found. Creating a new one...");
        gen_default_config(true)
    }
}

fn gen_default_config(save_to_file: bool) -> Config {
    // Default config values
    let default_config = Config {
        displayed_name: "Grasswave CDN".to_string(),
    };

    if save_to_file {
        // Serialize and save the file
        let default_config_toml = toml::to_string(&default_config).unwrap();
        fs::write("config.toml", default_config_toml).unwrap();
    }

    default_config
}

#[get("/")]
fn index() -> Template {
    let data = get_data();
    Template::render("index", context! {data: &data})
}

#[launch]
fn rocket() -> _ {
    // dbg!(get_config());
    println!("The server has started! Visit it at http://127.0.0.1:8000");

    rocket::build()
        .mount("/files", FileServer::from(relative!["/files"]))
        .mount("/static", FileServer::from(relative!["/static"]))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
