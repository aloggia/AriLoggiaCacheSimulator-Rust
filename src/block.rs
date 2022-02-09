

pub struct Block {
    //Because all addressing uses 16 bit numbers, that provides an upper constraint on the size of the
    // tag, and the address will always have aat least 1 offset bit, so we can confidently use a signed int
    tag: i16,
    //size of each block
    size: u8,
    //array to hold block memory
    mem: Vec<u8>,
    //bool dirty/clean
    is_dirty: bool,
    //bool valid/invalid
    is_valid: bool,
}

impl Block {
    pub fn new(size: u8) -> Block {
        Block {
            tag: -1,
            size,
            mem: Vec::with_capacity(size as usize),
            is_dirty: true,
            is_valid: false
        }
    }
    // get_tag, set_tag
    fn get_tag(&self) -> i16 {
        self.tag
    }
    fn set_tag(&mut self, new_tag: i16) {
        self.tag = new_tag;
    }
    // get_dirty, set_dirty
    fn get_dirty(&self) -> bool {
        self.is_dirty
    }
    fn set_dirty(&mut self, new_flag: bool) {
        self.is_dirty = new_flag;
    }
    // get_valid, set_valid
    fn get_valid(&self) -> bool {
        self.is_valid
    }
    fn set_valid(&mut self, new_flag: bool) {
        self.is_valid = new_flag;
    }
}