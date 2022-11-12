pub fn increment_cell(program: &mut [u8; 65_536], ptr: usize) {
    let value = program[ptr];
    program[ptr] = if value == 255 { 0 } else { value + 1 };
}

pub fn decrement_cell(program: &mut [u8; 65_536], ptr: usize) {
    let value = program[ptr];
    program[ptr] = if value == 0 { 255 } else { value - 1 };
}

pub fn advance_ptr_right(ptr: &mut usize) {
    ptr = if ptr == 65_535 { 0 } else { ptr + 1 };
}

pub fn advance_ptr_left(ptr: &mut usize) {
    ptr = if ptr == 0 { 65_535 } else { ptr - 1 };
}

pub fn write_char(program: &[u8; 65_536], ptr: usize) {
    todo!();
}

pub fn write_u8(program: &[u8; 65_536], ptr: usize) {
    print!("{}", program[ptr]);
}

pub fn read_char() {
    todo!();
}

pub fn read_u8() {
    todo!();
}

pub fn enter_loop() {
    todo!();
}