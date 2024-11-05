/* Mod Block
** - Função para calculo da Hash do bloco com base nos dados contidos no bloco
*/

use crate::Transaction;
use chrono::TimeZone;
use chrono_tz::Brazil::West;
use sha2::{Digest, Sha256};

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
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(id: u64, hash_previous_block: &str, transactions: Vec<Transaction>) -> Self {
        let dt = West.ymd(2024, 1, 1).and_hms(0, 0, 0);

        let timestamp = dt.timestamp() as u64;
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
        transactions: &Vec<Transaction>,
    ) -> String {
        let mut hasher = Sha256::new();

        // transformar todos os dados do bloco em uma única string
        let mut data = format!("{}{}{}", id, timestamp, hash_previous_block);

        //transformas os dados do vector transação em uma única string

        for tx in transactions {
            data.push_str(&format!("{}{}{}{}", tx.id, tx.from, tx.to, tx.value));
        }
        //Utilizando macro sha256 para criar a hash da variável data
        hasher.update(data);

        let result = format!("{:x}", hasher.finalize());
        result
    }
}
