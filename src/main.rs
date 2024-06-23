use clap::{arg, command, Parser};
use elden_ring_death_counter::read_save_file;
use std::{
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
    #[arg(short = 'F', long)]
    from: Option<i32>,
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

    if cli.from.is_none() && rand {
        println!("You might want to start from...");
        save.iter().enumerate().for_each(|(index, c)| {
            println!(
                "Save slot {} - character \"{}\" death {} times",
                index, c.name, c.death
            );
        })
    }

    for (slot, character) in save.iter().enumerate() {
        let path = Path::new(&cli.outdir);
        if !path.exists() {
            fs::create_dir_all(path).expect("unable to create directory");
        }
        let file_name = cli
            .outfile
            .replace("{slot}", &slot.to_string())
            .replace("{character_name}", &character.name);
        let path = path.join(file_name);
        let death = if cli.from.is_some() {
            0.max(character.death - cli.from.expect("unable to parse `from` option"))
        } else {
            character.death
        };
        let content = cli.format.replace("{}", &death.to_string());
        fs::write(path.clone(), content).expect("unable to write output");
    }

    println!("Done.")
}
