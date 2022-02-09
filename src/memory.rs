use bitlab::ExtractBitsFromIntegralTypes;

/*trait ByteManip {
    fn read
}
*/
//const MEMORY_SIZE: usize = 64;
pub struct Memory {
    size: usize,
    mem: Vec<u8>
}

impl Memory {
    fn new(capacity: usize) -> Memory {
        Memory {
            size: capacity,
            mem: Vec::with_capacity(capacity * 1000)
        }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    //pub fn write_byte
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.mem[addr as usize] = byte;
    }
    // pub fn read_word -> u32
    /*
    Read word takes in an addr, then calls read byte 4 times, and returns a u32 word
     */
    pub fn read_word(&self, addr: u16) -> u32 {
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
    pub fn write_word(&mut self, mut addr: u16, word: u32) {
        for pos in (0..=3).rev() {
            Memory::write_byte(self, addr, word.get_u8(8 * pos, 8).unwrap());
            addr += 1;
        }
    }
    fn get_size(&self) -> usize {
        self.size
    }
}

//Add assertion that memory[i] is the start of a word, ie memory[i] = i/4