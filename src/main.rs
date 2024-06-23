use std::{env, fs};

use byteorder::{BigEndian, ByteOrder};

struct Character {
    name: String,
    level: i32,
    seconds_played: i32,
    death: i32,
}

fn get_death_buffer_pos(data: &[u8]) -> isize {
    let mut index = 15000;
    while index < data.len().min(300000) {
        if data[index] == 255 {
            let mut count = 0;
            while index < data.len() && data[index] == 255 {
                count += 1;
                index += 1;
            }
            if count == 4 && data.get(index + 1) == Some(&8) {
                let mut found = false;
                let mut zero_count = 0;
                for offset in 2..47 {
                    if data.get(index + offset) == Some(&0) {
                        zero_count += 1;
                    }
                    if data.get(index + offset) == Some(&8) {
                        if zero_count >= 20 {
                            found = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
                if found {
                    return (index - 8) as isize;
                }
            }
        } else {
            index += 1;
        }
    }
    -1
}

fn get_zeros_from_binary(data: &[u8], min_length: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = -1;
    for (i, &item) in data.iter().enumerate() {
        if start == -1 && item == 0 {
            start = i as isize;
        } else if start != -1 {
            if item != 0 {
                if i - start as usize >= min_length {
                    ranges.push((start as usize, i - start as usize));
                }
                start = -1;
            }
        }
    }
    ranges
}

fn find_next_ff(data: &[u8], start: usize) -> isize {
    let mut index = -1;
    for (i, &item) in data.iter().enumerate().skip(start) {
        if index == -1 && item == 255 {
            index = i as isize;
        } else if index != -1 {
            if item == 255 && i as isize - index == 3 {
                return index;
            }
            if item != 255 {
                index = -1;
            }
        }
    }
    -1
}

static END_SLOT: usize = 0x280000;
static BEGIN_SLOT: usize = 0x1901d04;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("file path is not provided")
    };
    let path = &args[1];

    let buffer = fs::read(path).expect("unable to open save file");
    let mut save = Vec::new();

    for i in 0..10 {
        let start_bytes = 26221838 + 588 * i;
        let character_name_bytes = &buffer[start_bytes..start_bytes + 34];
        let character_name = String::from_utf16_lossy(
            &character_name_bytes
                .iter()
                .map(|&x| x as u16)
                .collect::<Vec<u16>>(),
        )
        .trim()
        .to_string();

        if character_name.is_empty() {
            continue;
        }

        let is_active = buffer[BEGIN_SLOT + i] == 1;
        if !is_active {
            continue;
        }

        let mut level_bytes = buffer[start_bytes + 34..start_bytes + 38].to_vec();
        level_bytes.reverse();
        let level = BigEndian::read_i32(&level_bytes);

        let mut seconds_played_bytes = buffer[start_bytes + 38..start_bytes + 42].to_vec();
        seconds_played_bytes.reverse();
        let seconds_played = BigEndian::read_i32(&seconds_played_bytes);

        let start_death_byte = 784 + 16 * i + i * END_SLOT;
        let death_buffer = &buffer[start_death_byte..start_death_byte + END_SLOT];
        let death_pos = get_death_buffer_pos(&death_buffer);
        let mut death_bytes = vec![];

        if death_pos != -1 && death_pos > 200000 && death_pos < 300000 {
            let death_pos = death_pos as usize;
            death_bytes = death_buffer[death_pos..death_pos + 4].to_vec();
        } else {
            let zeros = get_zeros_from_binary(&death_buffer, 50000);
            if !zeros.is_empty() {
                let t = zeros.iter().min_by_key(|&&(start, _)| start).unwrap();
                let next = find_next_ff(&death_buffer, t.0);
                if next > 200000 && next < 300000 {
                    let next_ff = next as usize;
                    death_bytes = death_buffer[next_ff - 4..next_ff].to_vec();
                }
            }
        }
        death_bytes.reverse();
        let death = BigEndian::read_i32(&death_bytes);

        save.push(Character {
            name: character_name.clone(),
            level,
            seconds_played,
            death,
        });
    }

    save.iter().for_each(|slot| {
        println!(
            "Character: {}\n - Level: {}\n - Played duration: {}\n - Death: {}",
            slot.name, slot.level, slot.seconds_played, slot.death
        );
    })
}
