use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use serde_derive::Deserialize;
use cached::proc_macro::once;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub displayed_name: String,
    pub files_path: PathBuf,
    pub accent_colors: [String; 2],
    pub http_port: u16,
}

impl Config {
    fn default(save_to_file: bool) -> Config {
        // Default config values
        let default_config = Config {
            displayed_name: "Grasswave CDN".to_string(),
            files_path: PathBuf::from("files"),
            accent_colors: [String::from("#1D9F00"), String::from("#4DE928")],
            http_port: 7000,
        };

        if save_to_file {
            // Serialize and save the file
            let default_config_toml = toml::to_string(&default_config).unwrap();
            fs::write("config.toml", default_config_toml).unwrap();
        }

        default_config
    }
}

#[once]
pub fn get_config() -> Config {
    let config_file = Path::new("config.toml");

    if config_file.is_file() {
        // Read, parse and return it
        let config_file_contents = fs::read_to_string(config_file).unwrap();
        match toml::from_str(&config_file_contents) {
            Ok(val) => val,
            Err(_) => {
                //  Use default values instead (but don't save them)
                eprintln!("Error: config file is not properly formatted");
                Config::default(false)
            },
        }
    } else {
        // Save and return a default config
        println!("No config file found. Creating a new one...");
        Config::default(true)
    }
}
