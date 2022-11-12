/// Function which validates that the character entered into the program
/// is one of 10 specific characters. The following characters, plus their
/// meanings, are included below.
/// 
/// * `>`: move pointer right by one
/// * `<`: move pointer left by one
/// * `+`: increment value at pointer by one
/// * `-`: decrement value at pointer by one
/// * `.`: output value at pointer as char
/// * `:`: output value at pointer as u8
/// * `,`: accept char as input, store at pointer as u8
/// * `;`: accept u8 as input, store at pointer
/// * `[`: enter loop
/// * `]`: exit loop
/// 
/// Returns true iff the input matches one of these characters, returns false otherwise
/// # Arguments
/// * `c`: a single character part of the inputted program file
/// 
pub fn valid_character(c: char) -> bool {
    match c {
        '>' |
        '<' |
        '+' |
        '-' |
        '.' |
        ':' |
        ',' |
        ';' |
        '[' |
        ']' => true,
        _ => false,
    }
}

/// Function that checks stores the indices of where a `[` or `]` character exists
/// in the program and returns two Vecs of usize if they occur in equal amounts.
/// 
/// Returns Ok((Vec, Vec)) iff the number of loop characters match, returns Err(String)
/// if they do not. The string contains an error message about the mismatched count
/// 
/// # Arguments
/// * program: a reference to the inputted program file
/// 
pub fn get_loop_brackets(program: &Vec<char>) -> Result<(Vec<usize>, Vec<usize>), String> {
    let (mut loop_starts, mut loop_ends) = (Vec::new(), Vec::new());
    for (idx, c) in program.iter().enumerate() {
        match *c {
            '[' => loop_starts.push(idx),
            ']' => loop_ends.push(idx),
            _ => continue,
        }
    }

    loop_ends.reverse();

    if loop_starts.len() == loop_ends.len() {
        Ok(
            (loop_starts, loop_ends)
        )
    } else {
        Err(
            format!("Error! Number of [ and ] characters in program do not match")
        )
    }
}