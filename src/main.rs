#[macro_use]
extern crate clap;
use clap::{App, Arg};

use std::fs::read_dir;
use std::fs::rename;
use std::path::{Path, PathBuf};

extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, OrderList, Select};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("An awesome show renamer")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY")
                .help("Sets a custom directory")
                .takes_value(true),
        )
        .get_matches();

    let directory = Path::new(matches.value_of("directory").unwrap_or("."));

    process_path(directory);
}

fn process_path(root: &Path) {
    println!("Processing {}", root.display());

    let shows = match read_dir(root) {
        Err(_) => panic!("Cannot read the root directory"),
        Ok(shows) => shows,
    };

    for show in shows {
        let show_path = match show {
            Err(_) => panic!("Cannot read the show directory"),
            Ok(show) => show.path(),
        };

        let show_name = show_path.file_name().unwrap().to_str().unwrap();
        println!("Processing the show: {}", show_name);

        if contains_files(&show_path) {
            println!("Aborting. Add season folders please.");
            continue;
        }

        let seasons = match read_dir(&show_path) {
            Err(_) => panic!("Cannot read the directory"),
            Ok(items) => items,
        };

        let mut season_names = vec![];

        for season in seasons {
            let season_path = match season {
                Err(_) => panic!("Cannot read the show directory"),
                Ok(show) => show.path(),
            };

            let season_name = season_path.file_name().unwrap().to_str().unwrap();
            season_names.push(season_name.to_string());
        }

        for i in 0..season_names.len() {
            let index = i + 1;
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&format!("Which one is season {} ?", index))
                .default(0)
                .items(&season_names)
                .interact()
                .unwrap();

            let folder_pattern = if index < 10 {
                format!("S0{}", index)
            } else {
                format!("S{}", index)
            };

            match rename(
                format!("{}/{}", &show_path.display(), season_names[selection]),
                format!("{}/{}", &show_path.display(), folder_pattern),
            ) {
                Err(_) => panic!("Cannot rename the folder"),
                _ => (),
            };
        }
    }
}

fn contains_files(path: &PathBuf) -> bool {
    let items = match read_dir(path) {
        Err(_) => panic!("Cannot read the directory"),
        Ok(items) => items,
    };

    items
        .map(|item| item.unwrap().path().is_file())
        .fold(false, |a, b| a || b)
}
