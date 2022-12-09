pub mod packets;

pub struct FrameHeader {
    pub size: u32,
}

pub struct Frame {
    header: FrameHeader,
    data: Vec<u8>,
}

impl Frame {
    pub fn new(data: Vec<u8>) -> Frame {
        let size = data.len() as u32;
        let cloned_data = data;
        let header = FrameHeader { size };

        Frame {
            header,
            data: cloned_data,
        }
    }
}


#[cfg(test)]
mod test {}
