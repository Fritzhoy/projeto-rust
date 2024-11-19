use blockchain::Blockchain;

mod block;
mod blockchain;
mod transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    // Criação de 20 transações para testar função da blockchain

    for i in 1..=20 {
        blockchain.create_transaction(
            "0xEf8801eaf234ff82801821FFe2d780237F9967",
            "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
            0.0 + i as f64,
        );
    }

    //checa se a cadeia de blocos é valida
    blockchain.is_chain_valid();

    //Chama a função que permite corromper um bloco
    blockchain.corrupt_block(2, 6, 2.53722);
}
