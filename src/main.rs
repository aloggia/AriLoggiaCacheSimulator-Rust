use crate::memory::Memory;

mod memory;
mod cache;
mod set;
mod block;

fn main() {
    //Global vars that control cache & memory properties
    const MEMORY_SIZE: u8 = 64;
    const CACHE_SIZE: u8 = 1;
    const BLOCK_SIZE: u8 = 64;
    const ASSOCIATIVITY: u8 = 4;

    let mut ram: Memory = Memory::new();

    let address: u16 = 65535;
    let block_number = (address - (address % 64)) / 64;

    println!("{}", block_number)


}

// Macro that ensures that the address is pointing to the start of the word
// If address isn't at the start of a word than set address to the start of the word it's in
#[macro_export]
macro_rules! align_address {
    ($addr:expr) => {
        {
            $addr - ($addr % 4)
        }
    }
}
