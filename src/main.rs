use elden_ring_death_counter::read_save_file;
use std::{env, fs, path::Path};

fn run(input_file: &str, format: &str, write_to: &str) -> std::io::Result<()> {
    let buffer = fs::read(input_file).expect("unable to open save file");
    let result = read_save_file(buffer);
    for character in result.iter() {
        let path = Path::new(write_to);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        let path = path.join(format!("{}.txt", character.name));
        let content = format.replace("{}", &character.death.to_string());
        fs::write(path.clone(), content)?;
    }
    drop(result);
    Ok(())
}

fn main() {
    let input_file = env::args().nth(1).expect("no save file provided");
    let output_folder = env::args().nth(2).unwrap_or("output".to_owned());
    let output_format = env::args().nth(3).unwrap_or("Death: {}".to_owned());

    let input = input_file.clone();
    let proceed =
        move || run(&input, &output_format, &output_folder).expect("unable to write output");

    proceed()
}
