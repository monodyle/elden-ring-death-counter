use byteorder::{BigEndian, ByteOrder};
use serde::Serialize;
use util::{find_next_ff, get_death_buffer_pos, get_zeros_from_binary};

mod util;

#[derive(Serialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub seconds_played: i32,
    pub death: i32,
}

static END_SLOT: usize = 0x280000;
static BEGIN_SLOT: usize = 0x1901d04;

pub fn read_save_file(buffer: Vec<u8>) -> Vec<Character> {
    let mut save = Vec::new();

    for i in 0..10 {
        let start_bytes = 26221838 + 588 * i;
        let character_name_bytes = &buffer[start_bytes..start_bytes + 34];
        let character_name = String::from_utf16_lossy(
            &character_name_bytes
                .iter()
                .map(|&x| x as u16)
                .filter(|&x| x != 0)
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

    save
}
