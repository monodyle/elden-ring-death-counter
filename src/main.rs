use clap::{arg, command, Parser};
use elden_ring_death_counter::{read_save_file, Character};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Elden Ring save file location
    input: String,
    /// Location will write death count files (default: "output")
    #[arg(short, long, default_value_t = String::from("output"))]
    outdir: String,
    /// Format of output files (defaut: "Death: {}"), where {} will be replaced by the death count
    #[arg(short, long, default_value_t = String::from("Death: {}"))]
    format: String,
    /// Death counter will start from this value instead of counting total character death
    #[arg(short = 'F', long, default_value_t = 0)]
    from: i32,
    /// Output filename format (default: {slot}-{character_name}.txt)
    #[arg(long, default_value_t = String::from("{slot}-{character_name}.txt"))]
    outfile: String,
}

fn main() {
    let cli = Cli::parse();

    let buffer = fs::read(cli.input).expect("unable to open save file");
    let save = read_save_file(buffer);
    let rand = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        % 10
        == 0;

    if rand {
        println!("You might want to start from...");
        save.iter().enumerate().for_each(|(index, c)| {
            println!(
                "Save slot {} - character \"{}\" death {} times",
                index, c.name, c.death
            );
        })
    }

    let get_cache_key =
        |slot: usize, c: &Character| format!("index_{}+name_{}+from_{}", slot, c.name, cli.from);

    /* cache to prevent the save file is writing so memory bytes at death position going to 0x0 */
    let cache = {
        let cache_path = Path::new(".cache");
        let mut cache = HashMap::new();
        if !cache_path.exists() {
            let content = save
                .iter()
                .enumerate()
                .map(|(slot, character)| {
                    let key = get_cache_key(slot, character);
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
                } else {
                }
            }
        }

        cache
    };

    let path = Path::new(&cli.outdir);
    if !path.exists() {
        fs::create_dir_all(path).expect("unable to create directory");
    }
    for (slot, character) in save.iter().enumerate() {
        let file_name = cli
            .outfile
            .replace("{slot}", &slot.to_string())
            .replace("{character_name}", &character.name);
        let path = path.join(file_name);
        let key = get_cache_key(slot, character);
        let death_cache = cache.get(&key);
        let mut death = 0.max(character.death - cli.from);
        if let Some(&death_cache) = death_cache {
            death = death.max(death_cache);
        }
        let content = cli.format.replace("{}", &death.to_string());
        fs::write(path.clone(), content).expect("unable to write output");
    }

    println!("Done.")
}
