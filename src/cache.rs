//Collection of sets
use bitlab::ExtractBitsFromIntegralTypes;
use crate::set::Set;
use crate::align_address;

pub struct Cache {
    address: u16,
    num_sets: u8,
    sets: Vec<Set>,
}

impl Cache {
    pub fn new(cache_size: u8, block_size: u8, associativity: u8) -> Cache {
        let num_of_blocks: u8 = cache_size / block_size;
        let num_of_sets: u8 = num_of_blocks / associativity;
        let mut set_vec: Vec<Set> = vec![];
        for set in 0..num_of_sets {
            set_vec.push(Set::new(associativity, block_size));
        }
        Cache {
            address: 0,
            num_sets: num_of_sets,
            sets: set_vec,
        }
    }
    /*
    Take in the address we want to find, and the size of a block in the cache
    Return a tuple consisting of: (block_tag, block_offset)
    In part 2 it'll be amended to (block_tag, block_index, block_offset)
     */
    fn address_as_tuple(addr: u16, block_size: u8) -> (u16, u16) {
        let mut offset_bits: u16 = 0;
        match block_size {
            32 => offset_bits = 5,
            64 => offset_bits = 6,
            128 => offset_bits = 7,
            _ => {}
        }
        let address_tuple: (u16, u16) = (addr.get_u16(0, offset_bits as u32).unwrap(),
                                         addr.get_u16(offset_bits as u32, 15).unwrap());
        address_tuple
    }

    // Read word function
    /*fn read_word(&self, addr: u16) -> u32 {
        align_address!(addr);
    }*/
    // Write word function
}