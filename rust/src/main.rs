use std::vec;

extern "C" {
    pub fn show(ptr: *const u8, len: usize);
}

fn main() {
    let data: Vec<u8> = vec![0x55, 0x44, 0x2]; // example
    unsafe { show(data.as_ptr(), data.len()) }
}
