use std::process::Command;
use chrono::prelude::*;
use config::Config;
use config_file::FromConfigFile;
use directories::UserDirs;

mod config;

fn main() {
    let hour = Local::now().hour();
    
    let proj_dirs = match UserDirs::new() {
        Some(dirs) => dirs,
        None => {
            print!("Cannot access to config dir");
            return
        }
    };

    let pictures = proj_dirs.picture_dir().unwrap().to_str().unwrap().to_owned();
    let home: String = proj_dirs.home_dir().to_str().unwrap().to_owned();
    
    let config = match Config::from_config_file( home + "/.config/Wallpaper/config.toml") {
        Ok(folders) => folders,
        Err(_) => Config {
            evening: pictures.clone() + "/Wallpapers/evening", 
            morning: pictures.clone() + "/Wallpapers/morning",
            night: pictures + "/Wallpapers/night",
        }
    };
    
    let mut folder = &config.morning;

    if hour > 7 {
        folder = &config.evening;
    }
    
    if hour > 19 {
        folder = &config.night;
    }
    
    let images = load_folder(folder);
    let image = rand::random::<usize>() % folder.len();
   
    change_wallpaper(&images[image]);
}


fn load_folder(folder: &str) -> Vec<String> {
    let mut result = Vec::new();

    let dir = match std::fs::read_dir(folder) {
        Ok(data) => data,
        Err(_) => return result,
    };
    
    for folder in dir {
        let folder = match folder {
            Ok(data) => data,
            Err(_) => continue,
        };

        match folder.path().to_str() {
            Some(data) => result.push(data.to_string()),
            _ => continue,
        };
    }
    
    return result;
}


fn change_wallpaper(img: &str) {
  let command = format!("/usr/bin/gsettings set org.gnome.desktop.background picture-uri-dark 'file:///{}'", img);
  run_command(&command);
}


fn run_command(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute process");
}
