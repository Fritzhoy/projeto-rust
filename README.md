# Projeto Construção de uma blockchain simples usando Rust;

Projeto cria uma blockchain simples utilizando rust, onde as funções principais encontra-se no
mod blockchain
Mod transaction: 
- Definição da Struct transaction

Mod Block:

- Definição da Struct Block
- Implementa funções: 
   - Novo bloco, instancia uma novo bloco
   - Calcula a Hash do bloco: Implementa a biblioteca sha2 do rust que implementa a função hash
     Nesse projeto utilizamos a sha-256.
     Como input essa função recebe os dados contidos no bloco: id, timestamp, previous hash, transactions
     e como output gera uma hash de 256-bit correspondente a criptografia dos dados inputados.
     A função hash é:
      - deterministica, ou seja, para uma mesma data inputada sempre gera a mesma hash como output.
      - Irreversível: Não é possível deduzir a entrada a partir da hash final
      - Alteração não entrada resulta em uma hash completamente diferente
      - Resistente a colisões: Improvável que duas entradas diferentes gerem o mesmo hash.
        
[Leia mais sobre SHA-256 no site do NIST](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)
  

 Mod Blockchain:
 - Instância uma nova blockchain e cria o bloco genesis
     - Bloco size: representa o tamanho de cada bloco
      - Pending_transaction: vetor que armazena de forma temporária
          as transações, até a mineração de um novo bloco
      - transaction_counter: contador de transações na blockchain, utilizado
          no transaction id.
 - A cada 5 transações criadas um novo bloco é minerado
 - Checa a validade da cadeia de blocos
    - Checa o id do bloco
    - Checa a hash anterior do bloco
    - Checa a hash criada a partir dos dados do bloco
 - Função para simular a corrupção do valor de um transação em um dado
bloco


Mod lib.rs contém código para compilação usando WebAssembly, no entanto não foi desenvolvido além nesse projeto.
- `wasm-runtime`: Código `no_std` que pode ser compilado para WebAssembly (a.k.a. `wasm32-unknown-unknown`)



## Copilar programa
- Certifique que o Rust esta instalado na sua maquina.
```

Rode os testes para garantir que esta tudo funcionando:
```sh
# Roda os testes de todos os projetos definidos no
# `Cargo.toml` desse workspace.
cd wasm-runtime
cargo test

#Roda o main para executar funções blockchain
cargo run

```

