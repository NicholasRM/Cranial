pub fn get_utf8_char(program: &[u8; 65_536], ptr: usize) -> Result<Option<char>, String> {
    let bytes = match get_codepoint(program[ptr]) {
        0..=7 => Ok([0,0,0,program[ptr]]),
        8..=11 => Err("Error! Invalid codepoint for beginning of UTF-8 character".to_owned()),
        12 | 13 => {
            if ptr + 1 >= 65_535 {
                Err("Error! Not enough contiguous bytes to store UTF-8 character".to_owned())
            } else {
                Ok([0,0,program[ptr],program[ptr+1]])
            }
        }
        14 => {
            if ptr + 2 >= 65_535 {
                Err("Error! Not enough contiguous bytes to store UTF-8 character".to_owned())
            } else {
                Ok([0, program[ptr], program[ptr+1], program[ptr+2]])
            }
        }
        15 => {
            if ptr + 3 >= 65_535 {
                Err("Error! Not enough contiguous bytes to store UTF-8 character".to_owned())
            } else {
                Ok([program[ptr], program[ptr+1], program[ptr+2], program[ptr+3]])
            }
        }
        _ => Err("Error! Codepoint exceeded range of 0-15".to_owned()),
    };

    match bytes {
        Ok(byte_arr) => Ok(char::from_u32(u32::from_be_bytes(byte_arr))),
        Err(msg) => Err(msg),
    }
}

pub fn put_utf8_char() {

}

fn get_codepoint(first_byte: u8) -> u8 {
    first_byte >> 4
}