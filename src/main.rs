mod app;
mod logger;
mod tokenizer;
mod highlighting;
mod language_pattern;
mod project_root;

#[macro_use]
extern crate lazy_static;

use app::{ Arguments, Config, App };
use colored::Colorize;
use project_root::get_project_root;

static VERSION: &str = "1.0.0";

fn main() {
    let project_root = get_project_root().unwrap().to_str().unwrap().to_string();
    match Arguments::from_env() {
        Ok(args) => {
            let config = Config::from_file(project_root.clone() + "/prettier.config.json");
            let app = App::new(VERSION, project_root, args, config);
            match app.run(0) {
                Ok(logger) => {
                    if logger.len() > 0 {
                        // there are some logs
                        println!("{}", logger);
                    }
                },
                Err(e) => {
                    // there is something wrong
                    println!("{}", e.red());
                }
            };
        },
        Err(e) => {
            println!("{}", e.red());
        }
    }
}
