use elden_ring_death_counter::{read_save_file, Character};
use notify::{
    event::{AccessKind, AccessMode},
    Error, Event, EventKind, Watcher,
};
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

    let buffer = fs::read(input_file.clone()).expect("unable to open save file");
    let result = read_save_file(buffer);

    let output_folder = env::args().nth(3).unwrap_or("output".to_owned());
    let output_format = env::args().nth(4).unwrap_or("Death: {}".to_owned());

    let write = move || {
        write_output(&result, &output_format, &output_folder, false)
            .expect("unable to write output")
    };

    match command.as_ref() {
        "debug" => write(),
        "get" => write(),
        "watch" => {
            write();

            let mut watcher =
                notify::recommended_watcher(move |res: Result<Event, Error>| match res {
                    Ok(event) => {
						println!("event: {:?}", event);
                        if event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                            write()
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                })
                .expect("unable to create a watcher");

            loop {
                watcher
                    .watch(Path::new(&input_file), notify::RecursiveMode::Recursive)
                    .expect("unable to watch input file");
            }
        }
        _ => panic!("invalid command '{command}', try again with: 'debug', 'get', or 'watch'"),
    }
}
