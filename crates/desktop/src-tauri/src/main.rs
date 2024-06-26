#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elden_ring_death_counter::{read_save_file, Character};
use once_cell::sync::Lazy;
use std::{fs, path::Path, sync::Mutex};

#[tauri::command]
fn load_save(location: &str) -> Vec<Character> {
    let buffer = fs::read(location).expect("unable to open save file");
    read_save_file(buffer)
}

static LAST_DEATH: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

#[tauri::command]
fn write_save(outdir: &str, filename: &str, death: i32, format: &str, from: i32) {
    let path = Path::new(&outdir);

    if !path.exists() {
        fs::create_dir_all(path).expect("unable to create directory");
    }
    let path = path.join(filename);
    let last_death = *LAST_DEATH.lock().expect("unable to get last death mutex");
    let mut death = death;
    death = death.max(last_death);
    if death != last_death {
        *LAST_DEATH.lock().expect("unable to get last death mutex") = death;
    }
    let content = format.replace("{}", &(death - from).to_string());
    fs::write(path.clone(), content).expect("unable to write output");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_save, write_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
