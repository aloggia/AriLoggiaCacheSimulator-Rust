use bitlab::ExtractBitsFromIntegralTypes;

pub struct Block {
    //Because all addressing uses 16 bit numbers, that provides an upper constraint on the size of the
    // tag, and the address will always have aat least 1 offset bit, so we can confidently use a signed int
    tag: i16,
    //size of each block
    size: u8,
    //array to hold block memory
    mem: [u8; size],
    //bool dirty/clean
    is_dirty: bool,
    //bool valid/invalid
    is_valid: bool,
}

impl Block {
    pub fn new(size: u8) {
        let mut block = Block {
            tag: -1,
            size,
            mem: [0, size],
            is_dirty: true,
            is_valid: false
        };
    }
    //Need a read_byte, write_byte, read_word, write_word
    // get_tag, set_tag
    // get_dirty, set_dirty
    // get_valid, set_valid
}