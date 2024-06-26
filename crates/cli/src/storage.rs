use std::{collections::HashMap, fs, path::Path};

use elden_ring_death_counter::Character;

pub struct Storage {
    pub data: HashMap<String, i32>,
}

impl Storage {
    const STORAGE_PATH: &'static str = ".storage";

    pub fn get_key(slot: usize, character: &Character, from: i32) -> String {
        format!("index_{}+name_{}+from_{}", slot, character.name, from)
    }

    pub fn init(save: &Vec<Character>, from: i32) -> Self {
        let mut storage = HashMap::new();
        let storage_path = Path::new(Self::STORAGE_PATH);
        if !storage_path.exists() {
            let content = save
                .iter()
                .enumerate()
                .map(|(slot, character)| {
                    let key = Self::get_key(slot, character, from);
                    storage.insert(key.clone(), character.death);
                    return format!("{} = {}", key, character.death);
                })
                .collect::<Vec<String>>()
                .join("\n");
            fs::write(storage_path, content).expect("unable to write new storage file");
        } else {
            let content = fs::read_to_string(storage_path).expect("unable to load storage file");
            for line in content.split("\n") {
                if let Some((key, value)) = line.trim().split_once(" = ") {
                    let value = value.parse::<i32>().expect("invalid data type");
                    storage
                        .entry(key.to_string())
                        .and_modify(|x| *x = value)
                        .or_insert(value);
                }
            }
        }

        Self { data: storage }
    }

    pub fn write_down(&mut self) {
        let content = self
            .data
            .iter()
            .map(|(&ref key, &value)| format!("{} = {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        let storage_path = Path::new(Self::STORAGE_PATH);
        fs::write(storage_path, content).expect("unable to write storage file");
    }

    pub fn flush() {
        let storage_path = Path::new(Self::STORAGE_PATH);
        if storage_path.exists() {
            fs::remove_file(storage_path).expect("unable to flush storage");
            println!("flushed storage");
        } else {
            println!("no storage file found, all clear")
        }
    }

    pub fn get_record(&mut self, key: String) -> Option<&i32> {
        self.data.get(&key)
    }

    pub fn update_record(&mut self, key: String, value: i32) -> &mut i32 {
        self.data
            .entry(key)
            .and_modify(|x| *x = value)
            .or_insert(value)
    }
}
