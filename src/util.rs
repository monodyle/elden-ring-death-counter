pub fn get_death_buffer_pos(data: &[u8]) -> isize {
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

pub fn get_zeros_from_binary(data: &[u8], min_length: usize) -> Vec<(usize, usize)> {
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

pub fn find_next_ff(data: &[u8], start: usize) -> isize {
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
