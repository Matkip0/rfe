use std::{env, fs, io};
use std::path::Path;
use chrono::{DateTime, Local};

struct Item{
    name: String,
    size: i32,
    modified: DateTime<Local>,
}

#[tauri::command]
pub fn read_directory(currentPath: &str) -> Vec<String> {
    let paths = fs::read_dir(Path::new(currentPath)).unwrap();

    let names =
        paths.map(|entry| {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let item_name = entry_path.file_name().unwrap();
            let item_name_as_str = item_name.to_str().unwrap();
            let item_name_as_string = String::from(item_name_as_str);

            println!("{}", item_name_as_string);
            item_name_as_string
        }).collect::<Vec<String>>();
    names
}