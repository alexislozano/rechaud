#[macro_use]
extern crate clap;
use clap::{App, Arg};

use std::fs::read_dir;
use std::fs::rename;

extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, OrderList};

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

    let directory = matches.value_of("directory").unwrap_or(".").to_string();

    process_path(directory);
}

fn process_path(root_path: String) {
    let show_names = folder_names(&root_path);

    for show_name in show_names {
        let show_path = format!("{}/{}", &root_path, &show_name);
        println!("Processing the show: {}", &show_name);

        if contains_files(&show_path) {
            println!("Aborting. Add season folders please.");
            continue;
        }

        let season_names = folder_names(&show_path);

        let order_seasons = if season_names.len() > 1 {
            OrderList::with_theme(&ColorfulTheme::default())
                .with_prompt("Please order the seasons")
                .items(&season_names)
                .interact()
                .unwrap()
        } else {
            vec![0]
        };

        for i in 0..season_names.len() {
            let old_season_name = &season_names[order_seasons[i]];
            match rename(
                format!("{}/{}", &show_path, old_season_name),
                format!("{}/{}_tmp", &show_path, old_season_name),
            ) {
                Err(_) => panic!("Cannot rename the folder"),
                _ => (),
            };
        }

        for i in 0..season_names.len() {
            let index_s = i + 1;

            let old_season_name = &season_names[order_seasons[i]];
            let new_season_name = if index_s < 10 {
                format!("S0{}", index_s)
            } else {
                format!("S{}", index_s)
            };

            match rename(
                format!("{}/{}_tmp", &show_path, old_season_name),
                format!("{}/{}", &show_path, new_season_name),
            ) {
                Err(_) => panic!("Cannot rename the folder"),
                _ => (),
            };

            println!("Processing the season: {}", &new_season_name);

            let season_path = format!("{}/{}", &show_path, &new_season_name);
            let episode_names = folder_names(&season_path);

            let order_episodes = if episode_names.len() > 1 {
                OrderList::with_theme(&ColorfulTheme::default())
                    .with_prompt("Please order the episodes")
                    .items(&episode_names)
                    .interact()
                    .unwrap()
            } else {
                vec![0]
            };

            for j in 0..episode_names.len() {
                let old_episode_name = &episode_names[order_episodes[j]];
                match rename(
                    format!("{}/{}", &season_path, old_episode_name),
                    format!("{}/{}_tmp", &season_path, old_episode_name),
                ) {
                    Err(_) => panic!("Cannot rename the folder"),
                    _ => (),
                };
            }

            for j in 0..episode_names.len() {
                let index_e = j + 1;

                let old_episode_name = &episode_names[order_episodes[j]];
                let old_episode_name_split: Vec<_> = old_episode_name.split(".").collect();
                let extension = old_episode_name_split[old_episode_name_split.len() - 1];
                let new_episode_name = if index_e < 10 {
                    format!("{}E0{}.{}", &new_season_name, index_e, extension)
                } else {
                    format!("{}E{}.{}", &new_season_name, index_e, extension)
                };

                match rename(
                    format!("{}/{}_tmp", &season_path, old_episode_name),
                    format!("{}/{}", &season_path, new_episode_name),
                ) {
                    Err(_) => panic!("Cannot rename the folder"),
                    _ => (),
                };
            }
        }
    }
}

fn contains_files(path: &str) -> bool {
    match read_dir(path) {
        Err(_) => panic!("Cannot read the directory"),
        Ok(mut items) => items.any(|item| item.unwrap().path().is_file()),
    }
}

fn folder_names(path: &str) -> Vec<String> {
    match read_dir(path) {
        Err(_) => panic!("Cannot read the directory"),
        Ok(items) => {
            let mut new_items = items
                .map(|item| {
                    item.unwrap()
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<String>>();
            new_items.sort();
            new_items
        }
    }
}
