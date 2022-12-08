pub struct Frame {
    size: u32,
    id: u16,
    data: Vec<u8>
}

impl Frame {
    pub fn new(id: u16, data: Vec<u8>) -> Frame {
        let size = data.len() as u32;
        let d = data.clone();
        Frame {size, id, data: d}
    }
}

