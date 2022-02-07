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

    // Create an array of size MEMORY_SIZE kilobytes
    // Each cells of memory holds 8 bits ie 1 byte, ie 1/4 of a word
    let mut memory: [u8; (MEMORY_SIZE * 1000) as usize];
}
