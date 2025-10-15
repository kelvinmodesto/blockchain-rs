pub mod blockchain {
    use blockchain::block::{Block, Result};
    use log::info;
    use sled::Db;

    #[derive(Debug, Clone)]
    pub struct Blockchain {
        current_hash: String,
        db: Db,
    }

    pub struct BlockchainIterator<'a> {
        hash: String,
        bc: &'a Blockchain,
    }

    impl Blockchain {
        pub fn new() -> Self {
            Blockchain {
                blocks: vec![Block::generate_block()],
            }
        }

        pub fn add_block(&mut self, data: String) -> Result<()> {
            let prev: &Block = self.blocks.last().unwrap();
            let new_block: Block =
                Block::new_block(data, prev.get_hash(), self.blocks.len() as i32)?;
            self.blocks.push(new_block);
            Ok(())
        }
    }
}
