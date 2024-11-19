/* Mod Blockchain:
** - Instância uma nova blockchain e cria o bloco genesis
**      - Bloco size: representa o tamanho de cada bloco
**      - Pending_transaction: vetor que armazena de forma temporária
**          as transações, até a mineração de um novo bloco
**      - transaction_counter: contador de transações na blockchain, utilizado
**          no transaction id.
** - A cada 5 transações criadas um novo bloco é minerado
** - Checa a validade da cadeia de blocos
**    - Checa o id do bloco
**    - Checa a hash anterior do bloco
**    - Checa a hash criada a partir dos dados do bloco
** - Função para simular a corrupção do valor de um transação em um dado
** bloco
* */

use crate::{block::Block, transaction::Transaction};
//use parity_scale_codec_derive::{Decode, Encode};

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct Blockchain {
    //Block genesis representa o primeiro bloco da blockchain
    pub chain: Vec<Block>,
    block_size: usize,
    pub pending_transactions: HashMap<u64, Transaction>,
    transaction_counter: u64,
}

impl Blockchain {
    //Função de criação da blockchain
    pub fn new() -> Self {
        //Ao criar uma nova blockchain, o block genesis é criado
        //Hardcode block_genesis
        let genesis_block = Block {
            id: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Erro ao obter o timestamp")
                .as_secs(),
            hash_previous_block: "0x000000000".to_string(),
            hash: "0x000000000".to_string(),
            transactions: HashMap::new(),
        };
        //variável mutável que representa o vetor da cadeia de blocos
        let mut chain = vec![];

        //block_genesis como primeiro elemento da cadeia de blocos da nova blockchain
        chain.push(genesis_block.clone());

        //tamanho do bloco fixo, indica que cada bloco pode conter 5 transações
        let block_size = 5;

        //contador de transações
        let transaction_counter = 1;

        //Vetor que armazena temporariamente as transações
        let pending_transactions = HashMap::new();

        let blockchain = Blockchain {
            chain,
            block_size,
            pending_transactions,
            transaction_counter,
        };

        println!(
            "Blockchain criada com sucesso! Bloco genesis:\n {:?}\n",
            &blockchain.chain[0]
        );
        blockchain
    }

    /* Ao completar 5 no pending_transactions, um novo bloco é minerado,
     ** sua hash é calculada e o bloco e adicionado a cadeia de blocos
     * */

    pub fn mine_block(&mut self) {
        let id = self.chain.len() as u64;
        /* Pega a hash do último bloco da cadeia é copia seu valor para o **previous_hash do  ** novo blo criado
         */
        let block_previous_hash = self.chain.last().unwrap().hash.clone();

        //Copia o vetor das pending_transactions, para o vetor transações do bloco
        let transactions = self.pending_transactions.clone();
        //Nova instância do tipo Blok
        let new_block = Block::new(id, &block_previous_hash, transactions);

        //Adiciona a blockchain o novo bloco instanciado.
        self.chain.push(new_block.clone());
        //Limpa o vetor de pending_transactions
        self.pending_transactions.clear();

        println!("Novo bloco adicionado a cadeia \n: {:?}\n", new_block);
    }
    // Função que instancia uma nova transação

    pub fn create_transaction(&mut self, from: &str, to: &str, value: f64) -> u64 {
        let transaction = Transaction {
            from: from.to_string(),
            to: to.to_string(),
            value,
        };
        let transaction_id = self.transaction_counter;

        self.pending_transactions
            .insert(transaction_id, transaction);

        if self.pending_transactions.len() == self.block_size {
            self.mine_block();
        }
        self.transaction_counter += 1;

        transaction_id
    }

    /* Checa a validade do bloco:
     ** Checa se previous_hash e a hash do bloco anterior são iguais
     ** Checa se o id do bloco é igual o id do bloco anterior +1
     ** Calcula a hash do current_block e checa se bate com a hash do cabeçalho do bloco
     */

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> Result<String, String> {
        if block.hash_previous_block != previous_block.hash {
            Err(String::from("Hash do Bloco Anterior incompatível"))
        } else if block.id != previous_block.id + 1 {
            Err(String::from("Não corresponde ao próximo bloco da cadeia"))
        } else if Block::calculate_block_hash(
            block.id,
            block.timestamp,
            &block.hash_previous_block,
            &block.transactions,
        ) != block.hash
        {
            Err(String::from("Hash invalida"))
        } else {
            Ok(String::from("valido"))
        }
    }
    /* Função checa a integridade da blockchain.
     ** Chama a função is_block_valid()
     * */

    pub fn is_chain_valid(&self) -> bool {
        /*
         ** checa a encadeação dos blocos, começando pelo primeiro bloco da cadeia,
         ** após o genesis_block, id: 1
         * */
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            let result = self.is_block_valid(&current_block, &previous_block);

            match result {
                Ok(_) => continue,
                Err(erro) => {
                    println!(
                        "Bloco id {}: {} Blockchain corrompida!",
                        current_block.id, erro
                    );
                    return false;
                }
            }
        }
        println!("Blockchain valida");
        true
    }
    // Possibilita a corrupção de uma dada transação em um dado bloco na blockchain

    pub fn corrupt_block(&mut self, block_id: usize, transaction_id: u64, new_value: f64) {
        // Checa a existência do bloco e da transação dentro do bloco

        if block_id < self.chain.len() && transaction_id <= self.transaction_counter {
            if let Some(corrupt_transaction) =
                self.chain[block_id].transactions.get_mut(&transaction_id)
            {
                corrupt_transaction.value = new_value;
                println!(
                    "Bloco id {} corrompido! Transação id: {} alterada! \nNovo valor: {:?}",
                    block_id, transaction_id, corrupt_transaction
                );
                // log(&format!(
                //     "Bloco id {} corrompido! Transação id: {} alterada! \nNovo valor: {:?}",
                //     block_id, transaction_id, corrupt_transaction
                // ));
            } else {
                // log(&format!(
                //     "Bloco id {} ou transação posição: {} não existe na cadeia de blocos",
                //     block_id, transaction_id
                // ));
                println!(
                    "Bloco id {} ou transação posição: {} não existe na cadeia de blocos",
                    block_id, transaction_id
                );
            }

            self.is_chain_valid();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_struct() {
        let mut blockchain = Blockchain::new();
        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }
        // checar tamanho da cadeia de blocos, considerando
        // a criação de 3 blocos além do genesis
        let chain_lenght = blockchain.chain;
        assert_eq!(chain_lenght.len(), 3);

        // teste criou 12 transações, ou seja deve existir 2 transações
        // na lista de transações pendentes.
        let pending_transaction_check = blockchain.pending_transactions;
        assert_eq!(pending_transaction_check.len(), 2);

        //checar tamanho do bloco
        let block_size_check = blockchain.block_size;
        assert_eq!(block_size_check, 5);

        //checar o transaction_counter, deve ser +1 número de transações criadas
        let transaction_counter_check = blockchain.transaction_counter;
        assert_eq!(transaction_counter_check, 13);
    }
    #[test]
    fn test_block_is_valid() {
        let mut blockchain = Blockchain::new();

        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }

        //Utiliza os dois últimos blocos da cadeia
        let current_block = blockchain.chain[2].clone();
        let previous_block = blockchain.chain[1].clone();

        //testa se os blocos são validos
        let result = blockchain.is_block_valid(&current_block, &previous_block);
        assert_eq!(result, Ok(String::from("valido")));
    }

    #[test]
    fn test_invalid_previous_hash() {
        let mut blockchain = Blockchain::new();

        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }

        //Utiliza os dois últimos blocos da cadeia
        let current_block = blockchain.chain[2].clone();
        let mut previous_block = blockchain.chain[1].clone();

        // alteração da hash do bloco anterior
        previous_block.hash = String::from("0x0000a52");

        // checa se a previous_hash do bloco atual é a mesma hash do bloco anterior,
        // deve retornar erro hash do bloco anterior incompatível
        let result = blockchain.is_block_valid(&current_block, &previous_block);
        assert_eq!(
            result,
            Err(String::from("Hash do Bloco Anterior incompatível"))
        );
    }
    #[test]
    fn test_invalid_block_id() {
        let mut blockchain = Blockchain::new();

        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }

        //Utiliza os dois últimos blocos da cadeia
        let mut current_block = blockchain.chain[2].clone();
        let previous_block = blockchain.chain[1].clone();

        //altera o id do bloco atual
        current_block.id = 5;

        //checar se retorna o erro de bloco não é o próximo da cadeia
        let result = blockchain.is_block_valid(&current_block, &previous_block);
        assert_eq!(
            result,
            Err(String::from("Não corresponde ao próximo bloco da cadeia"))
        );
    }
    #[test]

    fn test_invalid_hash() {
        let mut blockchain = Blockchain::new();

        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }

        //Utiliza os dois últimos blocos da cadeia
        let mut current_block = blockchain.chain[2].clone();
        let previous_block = blockchain.chain[1].clone();

        //Altera a hash do bloco atual
        current_block.hash = String::from("Invalid_hash");

        //Testa se a função retorna o erro de Hash invalida
        let result = blockchain.is_block_valid(&current_block, &previous_block);
        assert_eq!(result, Err(String::from("Hash invalida")));
    }

    #[test]
    fn test_is_chain_valid() {
        let mut blockchain = Blockchain::new();

        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }

        let result = blockchain.is_chain_valid();
        assert!(result);
    }
    #[test]
    fn test_is_chain_not_valid() {
        let mut blockchain = Blockchain::new();

        for i in 1..=12 {
            blockchain.create_transaction(
                "0xEf8801eaf234ff82801821FFe2d780237F9967",
                "0x889b8abc7aA5D9Ad5f7f531d68453f9984Fd6962",
                0.0 + i as f64,
            );
        }

        //Alterar um bloco na blockchain
        blockchain.corrupt_block(2, 6, 2.53722);

        //resultado deve retorna falso, bloco de id 2 foi corrompido
        let result = blockchain.is_chain_valid();
        assert!(!result);
    }
}
