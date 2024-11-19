/* Mod Block
** - Função para calculo da Hash do bloco com base nos dados contidos no bloco
*/
use crate::transaction::Transaction;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub struct Block {
    // identificação única do bloco
    pub id: u64,
    //timestamp do bloco
    pub timestamp: u64,
    //Hash do bloco
    pub hash: String,
    //Hash do bloco anterior
    pub hash_previous_block: String,
    //conjunto de transação incluídas no bloco
    pub transactions: HashMap<u64, Transaction>,
}

impl Block {
    pub fn new(
        id: u64,
        hash_previous_block: &str,
        transactions: HashMap<u64, Transaction>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Erro ao obter o timestamp")
            .as_secs() as u64;
        let hash = Block::calculate_block_hash(id, timestamp, &hash_previous_block, &transactions);

        Block {
            id,
            timestamp,
            hash: hash.to_string(),
            hash_previous_block: hash_previous_block.to_string(),
            transactions,
        }
    }

    //Função para calculo do block hash
    pub fn calculate_block_hash(
        id: u64,
        timestamp: u64,
        hash_previous_block: &str,
        transactions: &HashMap<u64, Transaction>,
    ) -> String {
        let mut hasher = Sha256::new();

        // transformar todos os dados do bloco em uma única string
        let mut data = format!("{}{}{}", id, timestamp, hash_previous_block);

        //transformas os dados do vector transação em uma única string

        for (tx_id, tx) in transactions {
            data.push_str(&format!("{}{}{}{}", tx_id, tx.from, tx.to, tx.value));
        }
        //Utilizando macro sha256 para criar a hash da variável data
        hasher.update(data);

        let result = format!("{:x}", hasher.finalize());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn test_calculate_block_hash() {
        let transactions = HashMap::from([
            (
                4,
                Transaction {
                    from: "0xEf8801eaf234ff82801821FFe2d780237F9967".to_string(),
                    to: "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962".to_string(),
                    value: 4.0,
                },
            ),
            (
                5,
                Transaction {
                    from: "0xEf8801eaf234ff82801821FFe2d780237F9967".to_string(),
                    to: "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962".to_string(),
                    value: 5.0,
                },
            ),
        ]);

        let test_block = Block::new(
            0,
            "a843dbfe51f762c3ca17f62633392c3a8538d45425d7f5ad1da877822c1073ae",
            transactions,
        );

        let hash = Block::calculate_block_hash(
            test_block.id,
            test_block.timestamp,
            &test_block.hash_previous_block,
            &test_block.transactions,
        );

        assert!(
            !hash.is_empty()
        );
    }
}
