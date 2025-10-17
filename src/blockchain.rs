pub mod blockchain {
    use blockchain::block::Block;

    use log::info;

    pub type Result<T> = std::result::Result<T, failure::Error>;

    const TARGET_HEXS: usize = 4;

    #[derive(Debug, Clone)]
    pub struct Blockchain {
        pub current_hash: String,
        pub db: sled::Db,
    }

    pub struct BlockchainIterator<'a> {
        current_hash: String,
        bc: &'a Blockchain,
    }

    impl Blockchain {
        pub fn new() -> Result<Blockchain> {
            info!("open blockchain");
            let db = sled::open("data/blocks")?;
            match db.get("LAST")? {
                Some(hash) => {
                    let last_hash = String::from_utf8(hash.to_vec())?;
                    Ok(Blockchain {
                        current_hash: last_hash,
                        db,
                    })
                }

                None => {
                    let block = Block::generate_block();
                    db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                    db.insert("LAST", block.get_hash().as_bytes())?;
                    let bc = Blockchain {
                        current_hash: block.get_hash(),
                        db,
                    };
                    bc.db.flush()?;
                    Ok(bc)
                }
            }
        }

        pub fn add_block(&mut self, data: String) -> Result<()> {
            let last_hash = self.db.get("LAST")?.unwrap();
            let new_block: Block = Block::new_block(
                data,
                String::from_utf8(last_hash.to_vec())?,
                TARGET_HEXS as i32,
            )?;

            self.db
                .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
            self.db.insert("LAST", new_block.get_hash().as_bytes())?;

            self.current_hash = new_block.get_hash();
            Ok(())
        }

        pub fn iter(&self) -> BlockchainIterator {
            BlockchainIterator {
                current_hash: self.current_hash.clone(),
                bc: &self,
            }
        }
    }

    impl<'a> Iterator for BlockchainIterator<'a> {
        type Item = Block;

        fn next(&mut self) -> Option<Self::Item> {
            if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
                return match encode_block {
                    Some(b) => {
                        if let Ok(block) = bincode::deserialize::<Block>(&b) {
                            self.current_hash = block.get_prev_hash();
                            Some(block)
                        } else {
                            None
                        }
                    }

                    None => None,
                };
            }

            None
        }
    }
}
