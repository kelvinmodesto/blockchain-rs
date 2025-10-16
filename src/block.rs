pub mod block {

    use std::time::SystemTime;

    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use log::info;
    use serde::Serialize;

    const TARGET_HEXS: usize = 4;
    pub type Result<T> = std::result::Result<T, failure::Error>;

    #[derive(Serialize, Debug, Clone)]
    pub struct Block {
        timestamp: u128,
        transactions: String,
        prev_block_hash: String,
        hash: String,
        nonce: i32,
        height: i32,
    }

    impl Block {
        pub fn generate_block() -> Block {
            Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
        }

        pub fn get_hash(&self) -> String {
            self.hash.clone()
        }

        pub fn new_block(data: String, prev_block_hash: String, height: i32) -> Result<Block> {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_millis();

            let mut block = Block {
                timestamp,
                transactions: data,
                prev_block_hash,
                hash: String::new(),
                height,
                nonce: 0,
            };
            block.run_proof_of_work()?;

            Ok(block)
        }

        fn prepare_hash_data(&self) -> Result<Vec<u8>> {
            let content = (
                self.prev_block_hash.clone(),
                self.transactions.clone(),
                self.timestamp,
                TARGET_HEXS,
                self.nonce,
            );

            let bytes = bincode::serialize(&content)?;
            Ok(bytes)
        }

        fn validate(&self) -> Result<bool> {
            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher: Sha256 = Sha256::new();
            hasher.input(&data[..]);
            let mut vec1: Vec<u8> = Vec::new();
            vec1.resize(TARGET_HEXS, '0' as u8);
            Ok(hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)
        }

        fn run_proof_of_work(&mut self) -> Result<()> {
            info!("Mining the block");
            while !self.validate()? {
                self.nonce += 1;
            }
            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher: Sha256 = Sha256::new();
            hasher.input(&data[..]);
            self.hash = hasher.result_str();
            Ok(())
        }
    }
}
