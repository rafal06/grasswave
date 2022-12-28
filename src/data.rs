use crate::config::get_config;

use std::fs;
use std::path::PathBuf;
use serde::Serialize;
use serde_derive::Deserialize;
use cached::proc_macro::once;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileData {
    name: String,
    description: String,
    tags: Vec<String>,
    path: PathBuf,
}

#[once(time=43200)]  // Update the cache once every 12h
pub fn get_data() -> Vec<FileData> {
    let files_dir = get_config().files_path;

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
                let entry_path = entry.file_name();
                item.path = PathBuf::from("files/");
                item.path.push(entry_path);
                item.path.push(item_path);
                dbg!(&item.path);

                // Push the CdnItem to the Vector
                files_arr.push(item);
            }
        }
    }

    files_arr
}
