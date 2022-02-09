//Collection of sets

use crate::set::Set;

pub struct Cache {
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
            num_sets: num_of_sets,
            //sets: vec![Set::new(associativity); size as usize],
            sets: set_vec,
        }
    }
}