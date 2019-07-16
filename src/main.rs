use clap::{crate_authors, crate_name, crate_version, App, Arg};
use dialoguer::{theme::ColorfulTheme, OrderList};
use std::fs::{read_dir, rename};

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
    println!("Processing the folder: {}", &root_path);
    let show_names = get_names(&root_path, false);

    for show_name in show_names {
        println!("Processing the show: {}", &show_name);
        let show_path = format!("{}/{}", &root_path, &show_name);
        let season_names = get_names(&show_path, false);

        let order_seasons = if season_names.len() > 1 {
            OrderList::with_theme(&ColorfulTheme::default())
                .with_prompt("Please order the seasons")
                .paged(true)
                .items(&season_names)
                .interact()
                .unwrap()
        } else {
            vec![0]
        };

        let mut error_seasons: Vec<bool> = (0..order_seasons.len()).map(|_| false).collect();

        for (i, order_season) in order_seasons.iter().enumerate() {
            let old_season_name = &season_names[*order_season];
            match rename(
                format!("{}/{}", &show_path, old_season_name),
                format!("{}/{}_tmp", &show_path, old_season_name),
            ) {
                Err(_) => {
                    eprintln!("Could not rename the folder");
                    error_seasons[i] = true;
                }
                Ok(_) => (),
            };
        }

        let width_season = format!("{}", order_seasons.len()).len();

        for (i, order_season) in order_seasons.iter().enumerate() {
            if error_seasons[i] {
                continue;
            }

            let index_s = i + 1;

            let old_season_name = &season_names[*order_season];
            let new_season_name = format!("S{:0width$}", index_s, width = width_season);

            match rename(
                format!("{}/{}_tmp", &show_path, old_season_name),
                format!("{}/{}", &show_path, new_season_name),
            ) {
                Err(_) => {
                    eprintln!("Could not rename the folder");
                    error_seasons[i] = true;
                }
                Ok(_) => (),
            };

            if error_seasons[i] {
                continue;
            }

            println!("Processing the season: {}", &new_season_name);
            let season_path = format!("{}/{}", &show_path, &new_season_name);
            let episode_names = get_names(&season_path, true);

            let order_episodes = if episode_names.len() > 1 {
                OrderList::with_theme(&ColorfulTheme::default())
                    .with_prompt("Please order the episodes")
                    .paged(true)
                    .items(&episode_names)
                    .interact()
                    .unwrap()
            } else {
                vec![0]
            };

            let mut error_episodes: Vec<bool> = (0..order_episodes.len()).map(|_| false).collect();

            for (j, order_episode) in order_episodes.iter().enumerate() {
                let old_episode_name = &episode_names[*order_episode];
                match rename(
                    format!("{}/{}", &season_path, old_episode_name),
                    format!("{}/{}_tmp", &season_path, old_episode_name),
                ) {
                    Err(_) => {
                        eprintln!("Could not rename the folder");
                        error_episodes[j] = true;
                    }
                    Ok(_) => (),
                };
            }

            let width_episode = format!("{}", order_episodes.len()).len();

            for (j, order_episode) in order_episodes.iter().enumerate() {
                if error_episodes[j] {
                    continue;
                }

                let index_e = j + 1;

                let old_episode_name = &episode_names[*order_episode];
                let old_episode_name_split: Vec<_> = old_episode_name.split(".").collect();
                let extension = old_episode_name_split[old_episode_name_split.len() - 1];
                let new_episode_name = format!(
                    "{}E{:0width$}.{}",
                    &new_season_name,
                    index_e,
                    extension,
                    width = width_episode
                );

                match rename(
                    format!("{}/{}_tmp", &season_path, old_episode_name),
                    format!("{}/{}", &season_path, new_episode_name),
                ) {
                    Err(_) => eprintln!("Could not rename the folder"),
                    Ok(_) => (),
                };
            }
        }
    }
}

fn get_names(path: &str, files_only: bool) -> Vec<String> {
    match read_dir(path) {
        Err(_) => {
            eprintln!("Could not process the folder");
            vec![]
        }
        Ok(items) => {
            let mut new_items = items
                .map(|item| item.unwrap().path())
                .filter(|path| !(files_only ^ path.is_file()))
                .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<String>>();
            new_items.sort();
            new_items
        }
    }
}
