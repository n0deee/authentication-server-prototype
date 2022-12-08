pub fn print_buffer(buf: &[u8]) {
    for n in buf.iter() {
        print!("{:02x} ", n);
    }
    print!("\n");
}
