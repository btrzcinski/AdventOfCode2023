use std::path::{Path, PathBuf};

use dialoguer::Select;

pub fn pick_data_file() -> PathBuf {
    let data_folder = Path::new("data");
    let mut choices: Vec<PathBuf> = data_folder
        .read_dir()
        .expect(&format!(
            "could not read from data folder: {}",
            data_folder.display()
        ))
        .filter_map(|r| r.ok().map(|e| e.path()))
        .collect();
    let displays: Vec<std::path::Display<'_>> = choices.iter().map(|p| p.display()).collect();
    let selection = Select::new()
        .with_prompt("Choose an input file")
        .items(&displays)
        .interact()
        .unwrap();
    choices.remove(selection)
}
