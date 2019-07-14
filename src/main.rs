use clap::{crate_authors, crate_name, crate_version, App, Arg};
use dialoguer::{theme::ColorfulTheme, OrderList};
use std::fs::read_dir;
use std::fs::rename;

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
    let show_names = get_names(&root_path, false);

    for show_name in show_names {
        let show_path = format!("{}/{}", &root_path, &show_name);
        println!("Processing the show: {}", &show_name);

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

        for order_season in order_seasons.iter() {
            let old_season_name = &season_names[*order_season];
            match rename(
                format!("{}/{}", &show_path, old_season_name),
                format!("{}/{}_tmp", &show_path, old_season_name),
            ) {
                Err(_) => panic!("Cannot rename the folder"),
                _ => (),
            };
        }

        let width_season = order_seasons.len() / 10 + 1;

        for (i, order_season) in order_seasons.iter().enumerate() {
            let index_s = i + 1;

            let old_season_name = &season_names[*order_season];
            let new_season_name = format!("S{:0width$}", index_s, width = width_season);

            match rename(
                format!("{}/{}_tmp", &show_path, old_season_name),
                format!("{}/{}", &show_path, new_season_name),
            ) {
                Err(_) => panic!("Cannot rename the folder"),
                _ => (),
            };

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

            for order_episode in order_episodes.iter() {
                let old_episode_name = &episode_names[*order_episode];
                match rename(
                    format!("{}/{}", &season_path, old_episode_name),
                    format!("{}/{}_tmp", &season_path, old_episode_name),
                ) {
                    Err(_) => panic!("Cannot rename the folder"),
                    _ => (),
                };
            }

            let width_episode = order_episodes.len() / 10 + 1;

            for (j, order_episode) in order_episodes.iter().enumerate() {
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
                    Err(_) => panic!("Cannot rename the folder"),
                    _ => (),
                };
            }
        }
    }
}

fn get_names(path: &str, files_only: bool) -> Vec<String> {
    match read_dir(path) {
        Err(_) => panic!("Cannot read the directory"),
        Ok(items) => {
            let mut new_items = items
                .map(|item| item.unwrap().path())
                .filter(|path| !(files_only ^ path.is_file()))
                .map(|path| path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                )
                .collect::<Vec<String>>();
            new_items.sort();
            new_items
        }
    }
}
