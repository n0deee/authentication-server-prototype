pub fn println_buffer(buf: &[u8]) {
    for n in buf.iter() {
        print!("{:02x} ", n);
    }
    print!("\n");
}

pub fn println_buffer_with_ascii(buf: &[u8]) {
    for n in buf.iter() {
        print!("{:02x} ", n);
    }
    print!("\n");
    for n in buf.iter() {
        let n = *n;

        const ASCII_FIRST_NON_CONTROL_CHARACTER: u8 = 32;
        let char: char = if n < ASCII_FIRST_NON_CONTROL_CHARACTER {
            '•'
        }
        else {
            n as char
        };

        print!("{}  ", char);
    }
    print!("\n");
}
