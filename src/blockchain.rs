use std::{
    hash,
    time::{SystemTime, UNIX_EPOCH},
};

// Criar
//Função de validação que percorre toda a blockchain e verifica integridade de cada bloco e seu relacionamento com o bloco anterior
use crate::{
    block::Block,
    transaction::{self, Transaction},
};

#[derive(Debug)]
pub struct Blockchain {
    //Block genesis representa o primeiro bloco da blockchain
    pub chain: Vec<Block>,
    block_size: usize,
    pending_transactions: Vec<Transaction>,
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
            transactions: vec![],
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
        let pending_transactions = vec![];

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

    /*
     **Ao completar 5 no pending_transactions, um novo bloco é minerado,
     **sua hash é calculada e o bloco e adicionado a cadeia de blocos
     */

    pub fn mine_block(&mut self) {
        let id = self.chain.len() as u64;
        /*Pega a hash do último bloco da cadeia é copia seu valor para o **previous_hash do novo blo criado*/
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
    /*Função que instancia uma nova transação */

    pub fn create_transaction(&mut self, from: &str, to: &str, value: f64) {
        let transaction = Transaction {
            id: self.transaction_counter,
            from: from.to_string(),
            to: to.to_string(),
            value,
        };

        self.pending_transactions.push(transaction);

        if self.pending_transactions.len() == self.block_size {
            self.mine_block();
        }
        self.transaction_counter += 1;
    }

    /*Checa a validade do bloco:
     **Checa se previous_hash e a hash do bloco anterior são iguais
     **Checa se o id do bloco é igual o id do bloco anterior +1
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
    /*
     **Função checa a integridade da blockchain.
     ** Chama a função is_block_valid()
     ** Verifica se os blocos estão encadeados corretamente, checa o id e
     ** recalcula o hash do bloco;
     */

    pub fn is_chain_valid(&self) {
        /*
         **checa a encadeação dos blocos, começando pelo primeiro bloco da cadeia, **após o genesis_block, id: 1
         */
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            let result = self.is_block_valid(&current_block, &previous_block);

            match result {
                Ok(resultado) => println!("Bloco id {} é {}", current_block.id, resultado),
                Err(erro) => println!("Bloco id {} : {}", current_block.id, erro),
            }
        }
    }
    /* Possibilita a corrupção de uma dado transação em um dado bloco na blockchain */

    pub fn corrupt_block(&mut self, block_id: usize, transaction_id: usize, new_value: f64) {
        //checa a existência do bloco e da transação dentro do bloco

        if block_id < self.chain.len() && transaction_id < self.chain[block_id].transactions.len() {
            self.chain[block_id].transactions[transaction_id].value = new_value;

            println!(
                "Bloco id {} corrompido! alterada transação:\n {:?} \n, novo valor: {}!",
                block_id, self.chain[block_id].transactions[transaction_id], new_value
            );
        } else {
            println!(
                "Bloco id {} ou transação posição: {} não existe na cadeia de blocos",
                block_id, transaction_id
            );
        }

        self.is_chain_valid();
    }
}
