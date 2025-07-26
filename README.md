# Blokchain_Rust
Uma implementação de uma blockchain simples em Rust, com foco em conceitos fundamentais como blocos, transações, mineração e segurança com criptografia de chave pública/privada (RSA) e hashing SHA-256.

## Sobre o Projeto
Este projeto foi desenvolvido como um estudo prático sobre a tecnologia blockchain, implementando seus principais componentes do zero. A simulação demonstra a criação de carteiras digitais, a assinatura de transações, a mineração de novos blocos e a validação da integridade da cadeia.

## Funcionalidades
Blockchain e Blocos: Estrutura de blockchain com uma cadeia de blocos interligados.

Mineração: Sistema de Prova de Trabalho (Proof-of-Work) para adicionar novos blocos à cadeia.

Transações Seguras: Transações com valores, remetentes e destinatários.

## Criptografia:

SHA-256: Implementação do algoritmo de hash para garantir a integridade dos blocos.

RSA: Geração de pares de chaves pública/privada para as carteiras e assinatura digital das transações.

Carteiras (Wallets): Criação de carteiras digitais para os participantes da rede.

Validação: Verificação da integridade da blockchain, incluindo a validação de hashes, prova de trabalho e assinaturas digitais.

Cálculo de Saldos: Funcionalidade para verificar o saldo de cada carteira na blockchain.

## Estrutura do Projeto
O projeto está organizado nos seguintes módulos:

main.rs: Orquestra a simulação da blockchain, criando carteiras, transações e blocos.

blockchain.rs: Define a estrutura e a lógica da blockchain, incluindo a adição e validação de blocos.

block.rs: Define a estrutura de um bloco individual.

transaction.rs: Define a estrutura de uma transação e a lógica para assinatura e verificação.

wallet.rs: Define a estrutura de uma carteira com chaves pública e privada.

rsa.rs: Implementação do algoritmo de criptografia RSA.

sha.rs: Implementação do algoritmo de hash SHA-256.

## Como Executar

### Compile e execute o projeto:

cargo run

A execução do main.rs iniciará uma simulação que irá:

Gerar as carteiras para os usuários Alice, Bob e Charlie.

Inicializar a blockchain e minerar o bloco gênesis.

Criar e assinar transações entre os usuários.

Minerar novos blocos para adicionar as transações à cadeia.

Calcular e exibir os saldos finais de cada carteira.

-----------------------------------------------------------------------------------------------------

A simple blockchain implementation in Rust, focusing on fundamental concepts like blocks, transactions, mining, and security with public/private key cryptography (RSA) and SHA-256 hashing.

## About the Project
This project was developed as a practical study of blockchain technology, implementing its main components from scratch. The simulation demonstrates the creation of digital wallets, the signing of transactions, the mining of new blocks, and the validation of the chain's integrity.

## Features
Blockchain and Blocks: A blockchain structure with a chain of interconnected blocks.

Mining: A Proof-of-Work system to add new blocks to the chain.

Secure Transactions: Transactions with values, senders, and recipients.

## Cryptography:

SHA-256: Implementation of the hash algorithm to ensure block integrity.

RSA: Generation of public/private key pairs for wallets and digital signing of transactions.

Wallets: Creation of digital wallets for network participants.

Validation: Verification of the blockchain's integrity, including the validation of hashes, proof-of-work, and digital signatures.

Balance Calculation: Functionality to check the balance of each wallet on the blockchain.

## Project Structure
The project is organized into the following modules:

main.rs: Orchestrates the blockchain simulation, creating wallets, transactions, and blocks.

blockchain.rs: Defines the blockchain's structure and logic, including adding and validating blocks.

block.rs: Defines the structure of an individual block.

transaction.rs: Defines the transaction structure and the logic for signing and verification.

wallet.rs: Defines the structure of a wallet with public and private keys.

rsa.rs: Implementation of the RSA cryptography algorithm.

sha.rs: Implementation of the SHA-256 hashing algorithm.

## How to Run
### Compile and run the project:
cargo run

Running main.rs will start a simulation that will:

Generate wallets for users Alice, Bob, and Charlie.

Initialize the blockchain and mine the genesis block.

Create and sign transactions between users.

Mine new blocks to add the transactions to the chain.

Calculate and display the final balances of each wallet

