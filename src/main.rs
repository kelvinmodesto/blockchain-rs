mod blockchain;
use blockchain::blockchain::Blockchain;

fn main() {
    let mut b: Blockchain = Blockchain::new();
    let _ = b.add_block("data1".to_string());
    let _ = b.add_block("data2".to_string());
    let _ = b.add_block("data3".to_string());
    dbg!(&b);
}
