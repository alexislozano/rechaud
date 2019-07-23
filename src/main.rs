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
        .arg(
            Arg::with_name("show")
                .short("s")
                .long("show")
                .help("Process a show directory. Otherwise, the directory\nis processed as a show containing directory."),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap_or(".");
    let show = matches.occurrences_of("show") == 1;

    if show {
        process_show(directory);
    } else {
        process_directory(directory);
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

fn process_directory(path: &str) {
    println!("Processing the folder: {}", path);
    let show_names = get_names(path, false);

    for show_name in show_names {
        let show_path = format!("{}/{}", path, show_name);
        process_show(&show_path);
    }
}

fn process_show(path: &str) {
    println!("Processing the show: {}", path);
    let season_names = get_names(path, false);

    let order_seasons = match season_names.len() {
        0 => vec![],
        1 => vec![0],
        _ => OrderList::with_theme(&ColorfulTheme::default())
            .with_prompt("Please order the seasons")
            .paged(true)
            .items(&season_names)
            .interact()
            .unwrap(),
    };

    let mut error_seasons: Vec<bool> = (0..order_seasons.len()).map(|_| false).collect();

    for (i, order_season) in order_seasons.iter().enumerate() {
        let old_season_name = &season_names[*order_season];
        match rename(
            format!("{}/{}", path, old_season_name),
            format!("{}/{}_tmp", path, old_season_name),
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
        let season_name = format!("S{:0width$}", index_s, width = width_season);

        match rename(
            format!("{}/{}_tmp", path, old_season_name),
            format!("{}/{}", path, season_name),
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

        let season_path = format!("{}/{}", path, season_name);
        process_season(&season_path, &season_name);
    }
}

fn process_season(path: &str, season_name: &str) {
    println!("Processing the season: {}", path);
    let episode_names = get_names(path, true);

    let order_episodes = match episode_names.len() {
        0 => vec![],
        1 => vec![0],
        _ => OrderList::with_theme(&ColorfulTheme::default())
            .with_prompt("Please order the episodes")
            .paged(true)
            .items(&episode_names)
            .interact()
            .unwrap(),
    };

    let mut error_episodes: Vec<bool> = (0..order_episodes.len()).map(|_| false).collect();

    for (j, order_episode) in order_episodes.iter().enumerate() {
        let old_episode_name = &episode_names[*order_episode];
        match rename(
            format!("{}/{}", path, old_episode_name),
            format!("{}/{}_tmp", path, old_episode_name),
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
        let episode_name = format!(
            "{}E{:0width$}.{}",
            season_name,
            index_e,
            extension,
            width = width_episode
        );

        match rename(
            format!("{}/{}_tmp", path, old_episode_name),
            format!("{}/{}", path, episode_name),
        ) {
            Err(_) => eprintln!("Could not rename the folder"),
            Ok(_) => (),
        };
    }
}
