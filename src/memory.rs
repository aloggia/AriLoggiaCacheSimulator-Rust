use bitlab::ExtractBitsFromIntegralTypes;
use crate::align_address;

/*trait ByteManip {
    fn read
}
*/
const ADDRESS_SIZE: usize = 16;


pub struct Memory {
    size: u32,
    mem: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            size: 2_u32.pow(ADDRESS_SIZE as u32) as u32,
            //2_i32.pow(ADDRESS_SIZE as u32) as u16,
            mem: vec![0; 2_u32.pow(ADDRESS_SIZE as u32) as usize]
            // Vec::with_capacity(2_u32.pow(ADDRESS_SIZE as u32) as usize)
        }
    }
    pub fn read_byte(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }
    //pub fn write_byte
    pub fn write_byte(&mut self, addr: u32, byte: u8) {
        self.mem[addr as usize] = byte;
    }
    // pub fn read_word -> u32
    /*
    Read word takes in an addr, then calls read byte 4 times, and returns a u32 word
     */
    pub fn read_word(&self, mut addr: u32) -> u32 {
        addr = align_address!(addr);
        let return_word: u32;
        return_word = (Memory::read_byte(self, addr) as u32) + 256 * ((Memory::read_byte(self,addr + 1) as u32) + 256 *
            ((Memory::read_byte(self,addr + 2) as u32) + 256 * (Memory::read_byte(self,addr + 3) as u32)));
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
    pub fn write_word(&mut self, mut addr: u32, word: u32) {
        addr = align_address!(addr);
        for pos in (0..=3).rev() {
            Memory::write_byte(self, addr, word.get_u8(8 * pos, 8).unwrap());
            addr += 1;
        }
    }
    pub fn get_size(&self) -> u32 {
        self.size
    }
}

//Add assertion that memory[i] is the start of a word, ie memory[i] = i/4