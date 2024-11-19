# Projeto Construção de uma blockchain simples usando Rust;

Projeto cria uma blockchain simples utilizando rust, criado 5 modulos diferentes, onde as funções principais encontra-se na
mod blockchain

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

