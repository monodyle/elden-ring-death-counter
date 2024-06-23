use elden_ring_death_counter::read_save_file;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("file path is not provided")
    };
    let path = &args[1];

    let buffer = fs::read(path).expect("unable to open save file");

    read_save_file(buffer).iter().for_each(|slot| {
        println!(
            "Character: {}\n - Level: {}\n - Played duration: {}\n - Death: {}",
            slot.name, slot.level, slot.seconds_played, slot.death
        );
    })
}
