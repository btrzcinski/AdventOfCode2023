use std::{path::{Path, PathBuf}, fs::File, io::Read, time::Instant};

use dialoguer::Select;

pub fn read_input_file(file_path: &Path) -> String {
    let display = file_path.display();
    let mut file = match File::open(file_path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => (),
    }
    return s;
}

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

pub fn report_runtime<F,R>(func: F) -> R 
    where F: FnOnce() -> R
{
    let now = Instant::now();
    let r = func();
    let elapsed_time = now.elapsed();

    println!("Running time: {:#?}", elapsed_time);
    r
}
