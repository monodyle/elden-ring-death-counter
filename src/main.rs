use clap::{arg, command, Parser};
use elden_ring_death_counter::read_save_file;
use std::{fmt::Debug, fs, path::Path};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Elden Ring save file location
    input: String,
    /// Location will write death count files (default: "output")
    #[arg(short, long)]
    outdir: Option<String>,
    /// Format of output files (defaut: "Death: {}"), where {} will be replaced by the death count
    #[arg(short, long)]
    format: Option<String>,
    /// Death counter will start from this value instead of counting total character death
    #[arg(short = 'F', long)]
    from: Option<i32>,
}

fn main() {
    let cli = Cli::parse();

    let output_folder = cli.outdir.unwrap_or(String::from("output"));
    let output_format = cli.format.unwrap_or(String::from("Death: {}"));

    let buffer = fs::read(cli.input).expect("unable to open save file");
    let save = read_save_file(buffer);

    if cli.from.is_some() {
        println!("Option \"from\" is provided, you might want to start from...");
        save.iter().enumerate().for_each(|(index, c)| {
            println!("Save slot {} - character \"{}\" death {} times", index, c.name, c.death);
        })
    }

    for (index, character) in save.iter().enumerate() {
        let path = Path::new(&output_folder);
        if !path.exists() {
            fs::create_dir_all(path).expect("unable to create directory");
        }
        let path = path.join(format!("{}-{}.txt", index, character.name));
        let death = if cli.from.is_some() {
            0.max(character.death - cli.from.expect("unable to parse `from` option"))
        } else {
            character.death
        };
        let content = output_format.replace("{}", &death.to_string());
        fs::write(path.clone(), content).expect("unable to write output");
    }

    println!("Done.")
}
