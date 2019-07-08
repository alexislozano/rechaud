#[macro_use]
extern crate clap;
use clap::{Arg, App};

use std::path::Path;
use std::fs::read_dir;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("An awesome show renamer")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .value_name("DIRECTORY")
            .help("Sets a custom directory")
                .takes_value(true))
        .get_matches();

    let directory = Path::new(matches
        .value_of("directory")
        .unwrap_or(".")
    );

    process_path(directory);
}

fn process_path(root: &Path) {
    println!("Processing {}", root.display());

    let shows = match read_dir(root) {
        Err(_) => panic!("Cannot read the root directory"),
        Ok(shows) => shows
    };

    for show in shows {
            
        let show_path = match show {
            Err(_) => panic!("Cannot read the show directory"),
            Ok(show) => show.path()
        };
        
        let show_name = show_path.file_name().unwrap().to_str().unwrap();
        println!("Processing the show: {}", show_name);

        let seasons = match read_dir(show_path) {
            Err(_) => panic!("Cannot read the directory"),
            Ok(items) => items
        };
        
        let contains_files: &bool = &seasons.map(
            |item| item.unwrap().path().is_file()
        ).fold(false, |a, b| a || b);

        // if *contains_files {
        //     println!("Aborting. Add season folders please.");
        //     continue;
        // }

        // println!("{:?}", seasons);
    }
}