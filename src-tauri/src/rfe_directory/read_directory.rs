use std::string::String;
use std::fs;
use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ItemType{
    File(File),
    Directory(Directory)
}
#[derive(Serialize, Deserialize)]
pub struct Item{
    name: String,
    modified: DateTime<Local>,
    item_type: ItemType,
    path: PathBuf,
}
#[derive(Serialize, Deserialize)]
pub struct File{
    file_type: String,
    size_in_bytes: u64,
}
#[derive(Serialize, Deserialize)]
pub struct Directory{
    item_count: usize,
}

#[tauri::command]
//TODO: fix snake case warning
pub fn read_directory(currentPath: &str) -> Result<Vec<Item>, &'static str> {
    let paths;
        match fs::read_dir(Path::new(currentPath)){
        Ok(prop) => paths = prop,
        Err(_) => return Err("invalid file path")
    };
    let mut items: Vec<Item> = Vec::new();

    for entry in paths{
        let item = read_dir_entry(entry.unwrap());
        match item { 
            Ok(prop) => items.push(prop),
            Err(_) => continue,
        }
    }
    Ok(items)
}

fn read_dir_entry(entry: DirEntry) -> Result<Item>{
    let entry_path = entry.path().canonicalize()?;
    let entry_metadata = entry_path.metadata()?;
    let item_name = match entry_path.file_name() {
        Some(prop) => prop,
        None => return Err(anyhow!("Error reading name"))
    };
    let item_name_as_str = match item_name.to_str() {
        Some(prop) => prop,
        None => return Err(anyhow!("Error converting to str"))
    };
    
    let item_name_as_string = String::from(item_name_as_str);
    let item_type = find_item_type(&entry_path, &entry_metadata).unwrap();
    let modified_date: DateTime<Local> = entry_metadata.modified()?.into();

    #[cfg(debug_assertions)]{
        println!("found item: {}", &item_name_as_string);
        // println!("{}", modified_date);
    }
    let new_item: Item = Item{
        name: item_name_as_string,
        modified: modified_date,
        item_type,
        path: entry_path
    };
    Ok(new_item)
}

fn find_item_type(entry_path: &PathBuf, entry_metadata: &Metadata) -> Result<ItemType, &'static str> {
    let item_type : ItemType;
    if entry_path.is_file(){
        item_type = ItemType::File(File{
            file_type: String::from(entry_path.extension().and_then(OsStr::to_str).unwrap_or("")),
            size_in_bytes: entry_metadata.size(),
        });
        return  Ok(item_type);
        
    } else if entry_path.is_dir() {
        item_type = ItemType::Directory(Directory{
            item_count: match read_dir(entry_path){
                Ok(prop) => prop.count(),
                Err(_) => 0
            },
        });
        return  Ok(item_type);
    }
    
    Err("item could not be validated")
}