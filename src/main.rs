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

    let mut new_addr: u32 = 43;
    println!("{}",new_addr);
    new_addr = align_address!(new_addr);
    println!("{}", new_addr);


}

// Macro that ensures that the address is pointing to the start of the word
// If address isnt at the start of a word than set address to the start of the word it's in
#[macro_export]
macro_rules! align_address {
    ($addr:expr) => {
        {
            $addr - ($addr % 4)
        }
    }
}
