mod storage;

use elden_ring_death_counter::read_save_file;
use storage::Storage;
use clap::{arg, command, Parser};
use std::{
    fmt::Debug,
    fs,
    path::Path,
    process,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Elden Ring save file location
    input: Option<String>,
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
    /// Clear all previous storage, this option should be using standalone
    #[arg(long, default_value_t = false)]
    flush: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.flush {
        Storage::flush();
        process::exit(0);
    }

    if cli.input == None {
        panic!("no save file provided");
    }
    let buffer = fs::read(cli.input.unwrap()).expect("unable to open save file");
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

    /* storage to prevent the save file is writing so memory bytes at death position going to 0x0 */
    let mut storage = Storage::init(&save, cli.from);

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
        let key = Storage::get_key(slot, character, cli.from);
        let death_storage = storage.get_record(key.clone());
        let mut death = 0.max(character.death - cli.from);
        if let Some(&death_storage) = death_storage {
            death = death.max(death_storage);
            if death != death_storage {
                storage.update_record(key, death);
            }
        }
        let content = cli.format.replace("{}", &death.to_string());
        fs::write(path.clone(), content).expect("unable to write output");
    }

	storage.write_down();
    println!("Done.")
}
