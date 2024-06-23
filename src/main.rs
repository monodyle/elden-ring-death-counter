use elden_ring_death_counter::{read_save_file, Character};
use std::{env, fs, path::Path};

fn write_output(
    save: &Vec<Character>,
    format: &str,
    write_to: &str,
    debug: bool,
) -> std::io::Result<()> {
    for character in save.iter() {
        let path = Path::new(write_to);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        let path = path.join(format!("{}.txt", character.name));
        let content = format.replace("{}", &character.death.to_string());
        fs::write(path.clone(), content.clone())?;
        if debug {
            println!(
                "write file '{}' content:\n{:?}",
                path.into_os_string()
                    .into_string()
                    .expect("unable to get output file path"),
                content
            );
            println!();
        }
    }

    Ok(())
}

fn main() {
    let command = env::args()
        .nth(1)
        .expect("unknown command, try: 'debug', 'get', or 'watch'");
    let input_file = env::args().nth(2).expect("no save file provided");

    let buffer = fs::read(input_file).expect("unable to open save file");
    let result = read_save_file(buffer);

    let output_folder = env::args().nth(3).unwrap_or("output".to_owned());
    let output_format = env::args().nth(4).unwrap_or("Death: {{}}".to_owned());
    match command.as_ref() {
        "debug" => write_output(&result, &output_format, &output_folder, true)
            .expect("unable to write output"),
        "get" => write_output(&result, &output_format, &output_folder, false)
            .expect("unable to write output"),
        "watch" => write_output(&result, &output_format, &output_folder, false)
            .expect("unable to write output"),
        _ => panic!("invalid command '{command}', try again with: 'debug', 'get', or 'watch'"),
    }
}
