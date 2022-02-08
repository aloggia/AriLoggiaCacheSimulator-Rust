use bitlab::ExtractBitsFromIntegralTypes;
use std::ptr::write_bytes;


pub struct Memory {
    size: u8,
    mem: [u8; (size * 1000) as usize],
}

impl Memory {
    pub fn new(capacity: u8) -> Memory {
        let mut memory = Memory {
            size: capacity,
            mem: [0, (capacity * 1000)],
        };
    }
    pub fn read_byte(&self, addr: usize) -> u8 {
        self.mem[addr]
    }
    //pub fn write_byte
    pub fn write_byte(&mut self, addr: usize, byte: u8) {
        self.mem[addr] = byte;
    }
    // pub fn read_word -> u32
    /*
    Read word takes in an addr, then calls read byte 4 times, and returns a u32 word
     */
    pub fn read_word(&self, addr: usize) -> u32 {
        let mut return_word: u32;
        return_word = read_byte(addr) + 256 * (read_byte(addr + 1) + 256 * (read_byte(addr + 2) + 256 * read_byte(addr + 3)));
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
    pub fn write_word(&mut self, mut addr: usize, word: u32) {
        for pos in (0..=3).rev() {
            write_byte(addr, word.get_u8((8 * pos), 8).unwrap());
            addr += 1;
        }
    }
}