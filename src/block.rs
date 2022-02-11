use bitlab::ExtractBitsFromIntegralTypes;
use crate::align_address;

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
            //mem: Vec::with_capacity(size as usize),
            mem: vec![0; size as usize],
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
    fn read_byte(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }
    //pub fn write_byte
    fn write_byte(&mut self, addr: u32, byte: u8) {
        self.mem[addr as usize] = byte;
    }
    // pub fn read_word -> u32
    /*
    Read word takes in an addr, then calls read byte 4 times, and returns a u32 word
     */
    fn read_word(&self, mut addr: u32) -> u32 {
        addr = align_address!(addr);
        let return_word: u32;
        return_word = (self.read_byte(addr) as u32) + 256 * ((self.read_byte(addr + 1) as u32) + 256 *
            ((self.read_byte(addr + 2) as u32) + 256 * (self.read_byte(addr + 3) as u32)));
        return return_word
    }
    //pub fn write_word
    /*
    Take in a 32 bit word and an address to write it too
    call write_byte 4 times starting at specified memory location
    For each call take the specific byte and write it to addr
    Use little endian, so start at bit offset 24, then offset 16, then 8, then 0
    increment addr by 1 to write to the next memory cell
     */
    fn write_word(&mut self, mut addr: u32, word: u32) {
        addr = align_address!(addr);
        for pos in (0..=3).rev() {
            self.write_byte(addr, word.get_u8(8 * pos, 8).unwrap());
            addr += 1;
        }
    }
}