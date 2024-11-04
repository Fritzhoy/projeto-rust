use blockchain::Blockchain;
use transaction::Transaction;

mod block;
mod blockchain;
mod transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    for i in 1..=20 {
        blockchain.create_transaction(
            "0xEf8801eaf234ff82801821FFe2d780237F9967",
            "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
            2.53722 * i as f64,
        );
    }

    blockchain.is_chain_valid();

    // blockchain.chain[3].hash = String::from("hello");
    // println!("{:?}", blockchain.chain[3]);

    blockchain.corrupt_block(3, 2, 1000.0);
}
