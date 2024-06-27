use std::process::Command;
use chrono::prelude::*;


const FOLDERS: [&str; 3] = [
    "/media/data/sync/out/wallpapers/night",
    "/media/data/sync/out/wallpapers/morning",
    "/media/data/sync/out/wallpapers/evening",
];


fn main() {
    let hour = Local::now().hour();

    let mut folder = FOLDERS[0];

    if hour > 7 {
        folder = FOLDERS[1];
    }
    
    if hour > 19 {
        folder = FOLDERS[2];
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
