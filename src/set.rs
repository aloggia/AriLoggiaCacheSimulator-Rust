use crate::block::Block;

// Collection of blocks
//#[derive(Clone)]
pub struct Set {
    associativity: u8,
    tag_queue: Vec<i16>,
    blocks: Vec<Block>
}
/*impl Clone for Set {
    fn clone(&self) -> Self {
        Set {
            associativity: self.associativity,
            tag_queue: self.tag_queue,
            blocks: self.blocks,
        }
    }
}*/
impl Set {
    pub fn new(associativity: u8, block_size: u8) -> Set {
        let mut block_set: Vec<Block> = vec![];
        for block in 0..associativity {
            block_set.push(Block::new(block_size));
        }
        Set {
            associativity,
            tag_queue: vec![-1, (associativity as i8).into()],
            blocks: block_set,
        }
    }
    // Impl copy constructor
    /* TODO:
    1. Function to move a block into the set - check
    2. Function to check the tag queue for a block - check
    3. Function to update the tag queue - TODO IN PART TWO
     */
    fn move_in (&mut self, move_block: Block) {
        // check the tag_queue for the oldest block - don't need to do yet
        // Copy the data from move_block to the fields of the block we're overwriting
        self.blocks[0] = move_block;

    }

    fn check_tag (&self, tag_to_check: i16) -> bool {
        if self.tag_queue[0] == tag_to_check { true } else { false }

    }
    pub(crate) fn get_block(&mut self, block_number: usize) -> &Block {
        &self.blocks[block_number]
    }

}