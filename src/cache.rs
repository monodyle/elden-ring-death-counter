use std::{collections::HashMap, fs, path::Path};

use elden_ring_death_counter::Character;

pub struct Cache {
    pub data: HashMap<String, i32>,
}

impl Cache {
    const CACHE_PATH: &'static str = ".cache";

    pub fn get_cache_key(slot: usize, character: &Character, from: i32) -> String {
        format!("index_{}+name_{}+from_{}", slot, character.name, from)
    }

    pub fn init(save: &Vec<Character>, from: i32) -> Self {
        let mut cache = HashMap::new();
        let cache_path = Path::new(Self::CACHE_PATH);
        if !cache_path.exists() {
            let content = save
                .iter()
                .enumerate()
                .map(|(slot, character)| {
                    let key = Self::get_cache_key(slot, character, from);
                    cache.insert(key.clone(), character.death);
                    return format!("{} = {}", key, character.death);
                })
                .collect::<Vec<String>>()
                .join("\n");
            fs::write(cache_path, content).expect("unable to write new cache file");
        } else {
            let content = fs::read_to_string(cache_path).expect("unable to load cache file");
            for line in content.split("\n") {
                if let Some((key, value)) = line.trim().split_once(" = ") {
                    let value = value.parse::<i32>().expect("invalid data type");
                    cache
                        .entry(key.to_string())
                        .and_modify(|x| *x = value)
                        .or_insert(value);
                }
            }
        }

        Self { data: cache }
    }

    pub fn write_down(&mut self) {
        let content = self
            .data
            .iter()
            .map(|(&ref key, &value)| format!("{} = {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        let cache_path = Path::new(Self::CACHE_PATH);
        fs::write(cache_path, content).expect("unable to write cache file");
    }

    pub fn flush() {
        let cache_path = Path::new(Self::CACHE_PATH);
        if cache_path.exists() {
            fs::remove_file(cache_path).expect("unable to flush cache");
            println!("flushed cache");
        } else {
            println!("no cache file found, all clear")
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
