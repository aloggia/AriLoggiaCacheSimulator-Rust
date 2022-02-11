//Collection of sets
use bitlab::ExtractBitsFromIntegralTypes;
use crate::set::Set;
use crate::memory::Memory;
use crate::align_address;

pub struct Cache {
    block_size: u8,
    num_sets: u8,
    sets: Vec<Set>,
    is_write_back: bool,
    memory: Memory,
}

impl Cache {
    pub fn new(cache_size: u8, block_size: u8, associativity: u8, write_back: bool) -> Cache {
        let num_of_blocks: u8 = cache_size / block_size;
        let num_of_sets: u8 = num_of_blocks / associativity;
        let mut set_vec: Vec<Set> = vec![];
        for set in 0..num_of_sets {
            set_vec.push(Set::new(associativity, block_size));
        }
        Cache {
            block_size,
            num_sets: num_of_sets,
            sets: set_vec,
            is_write_back: write_back,
            memory: Memory::new(),
        }
    }
    /*
    Take in the address we want to find, and the size of a block in the cache
    Return a tuple consisting of: (block_tag, block_offset)
    In part 2 it'll be amended to (block_tag, block_index, block_offset)
     */
    pub fn address_as_tuple(addr: u16, block_size: u8) -> (i16, u16) {
        let mut offset_bits: u16 = 0;
        match block_size {
            32 => offset_bits = 5,
            64 => offset_bits = 6,
            128 => offset_bits = 7,
            _ => {}
        }
        let address_tuple: (i16, u16) = ((addr.get_u16(0, offset_bits as u32).unwrap() as i16),
                                         addr.get_u16(offset_bits as u32, 15).unwrap());
        address_tuple
    }

    // Read word function
    fn read_word_from_cache(&self, addr: u16) -> u32 {
        let mut offset: u16 = 0;
        let mut tag: i16 = 0;
        let mut address_tuple: (i16, u16) = (tag, offset);
        let block_number: u16 = (addr - (addr % 64)) / 64;
        let set_num: u16 = block_number % (self.sets.len() as u16);
        address_tuple = Cache::address_as_tuple(addr, self.block_size);
        self.get_set(set_num as usize).get_block(0).read_word(addr as u32)
            //[0].mem[addr]
    }
    // Write word function
    fn write_word_to_cache(&mut self, addr: u16, word: u32) {
        let mut offset: u16 = 0;
        let mut tag: i16 = 0;
        let mut address_tuple: (i16, u16) = (tag, offset);
        let block_number: u16 = (addr - (addr % 64)) / 64;
        let set_num: u16 = block_number % (self.sets.len() as u16);
        address_tuple = Cache::address_as_tuple(addr, self.block_size);
        /*
        This line causes error, cannot borrow as mutable
        Idea: create a copy of the set, make the writes you want, than copy it back into the cache
         */
        self.sets[set_num as usize].get_block(0).write_word(addr as u32, word);
    }
    fn get_set(&self, set_num: usize) -> &Set {
        &self.sets[set_num]
    }
    // General function that calls read/write word depending on the instruction

    //pub fn read_word(addr: u16) -> u32 {

    //}

    pub fn write_word(addr: u16, word: u32) {

    }

}