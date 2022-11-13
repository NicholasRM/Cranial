use crate::formatting;
use std::io;

/// Function to increment the value of a byte at the pointer.
/// 
/// If the value of the byte is 255 (0b11111111), the value is set to 0.
/// Otherwise, the value is incremented by 1.
/// 
/// # Arguments:
/// * `tape`: A mutable reference to the tape of bytes used in the program
/// * `ptr`: the value of the data pointer at the time of being called
/// 
pub fn increment_cell(tape: &mut [u8; 65_536], ptr: usize) {
    let value = tape[ptr];
    tape[ptr] = if value == 255 { 0 } else { value + 1 };
}

/// Function to decrement the value of a byte at the pointer.
/// 
/// If the value of the byte is 0 (0b00000000), the value is set to 255.
/// Otherwise, the value is decremented by 1.
/// 
/// # Arguments:
/// * `tape`: A mutable reference to the tape of bytes used in the program
/// * `ptr`: the value of the data pointer at the time of being called
/// 
pub fn decrement_cell(tape: &mut [u8; 65_536], ptr: usize) {
    let value = tape[ptr];
    tape[ptr] = if value == 0 { 255 } else { value - 1 };
}

/// Function to move the pointer to the right by 1.
/// 
/// If the pointer is at the last index (65,535), the pointer is set to 0.
/// Otherwise, the value of the pointer is incremented by 1.
/// 
/// # Arguments:
/// * `ptr`: a mutable reference to the data pointer at the time of being called
/// 
pub fn advance_ptr_right(ptr: &mut usize) {
    ptr = if ptr == 65_535 { 0 } else { ptr + 1 };
}

/// Function to move the pointer to the left by 1.
/// 
/// If the pointer is at the first index (0), the pointer is set to 65,535.
/// Otherwise, the value of the pointer is decremented by 1.
/// 
/// # Arguments:
/// * `ptr`: a mutable reference to the data pointer
/// 
pub fn advance_ptr_left(ptr: &mut usize) {
    ptr = if ptr == 0 { 65_535 } else { ptr - 1 };
}

/// Function to write a UTF-8 character to standard output.
/// 
/// If the section of bytes encounters an invalid starting codepoint (0b1000 - 0b1011),
/// there is not enough bytes to the right of the pointer to read the full character,
/// or if the bytes do not make a valid UTF-8 character, an appropriate error message is
/// written to standard error and the program terminates with exit code 1.
/// 
/// If the program is able to generate a valid UTF-8 character, it is written to standard output.
/// 
/// # Arguments:
/// * `tape`: a reference to the tape of bytes used by the program
/// * `ptr`: the value of the data pointer at the time of being called
/// 
pub fn write_char(tape: &[u8; 65_536], ptr: usize) {
    let letter = formatting::get_utf8_char(tape, ptr);
    match letter {
        Ok(Some(c)) => print!("{c}"),
        Ok(None) => {
            eprintln!("Error! Invalid UTF-8 character produced");
            std::process::exit(1);
        }
        Err(msg) => {
            eprintln!("{s}");
            std::process::exit(1);
        }
    }
}

/// Function to write a 1 byte unsigned number to standard output.
/// 
/// # Arguments:
/// * `tape`: a reference to the tape of bytes used by the program
/// * `ptr`: the value of the data pointer at the time of being called
/// 
pub fn write_u8(tape: &[u8; 65_536], ptr: usize) {
    print!("{}", tape[ptr]);
}

pub fn read_char(tape: &mut [u8; 65_536], ptr: usize) {
    todo!();
}

pub fn read_u8(tape: &mut [u8; 65_536], ptr: usize) {
    todo!();
}

pub fn enter_loop(loop_starts: &Vec<usize>, loop_ends: &Vec<usize>, tape: &[u8; 65_536], ptr: &mut usize) {
    let loop_start_idx;
    for idx in loop_starts.iter() {
        if *idx == ptr {
            loop_start_idx = *idx;
        }
    }

    if tape[ptr] == 0 {
        ptr = loop_ends.get_mut(loop_start_idx).unwrap();
    } else {
        advance_ptr_right(ptr);
    }
}

pub fn exit_loop(loop_starts: &Vec<usize>, loop_ends: &Vec<usize>, tape: &[u8; 65_536], ptr: &mut usize) {
    let loop_end_idx;
    for idx in loop_ends.iter() {
        if *idx == ptr {
            loop_end_idx = *idx;
        }
    }

    if tape[ptr] != 0 {
        ptr = loop_starts.get_mut(loop_end_idx).unwrap();
    } else {
        advance_ptr_right(ptr);
    }
}

fn get_utf8_input() -> Result<[u8; 4], String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error! Failed to read input from user");
    if input.len() > 1 {
        Err("Error! Character input may not exceed 1 character in length".to_owned())
    } else {
        Ok(u32::from_be_bytes(input[0] as u32))
    }
}