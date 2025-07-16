use crate::block::Block;
use crate::transaction::Transaction;
use crate::transaction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: HashMap<String, Block>,
    pub last: String,
    pub difficulty: String,
}


impl Blockchain{
    pub fn new() -> Self{
        let mut chain: HashMap<String, Block> = HashMap::new();
        let difficulty = String::from("0000");

        let mut genesis_block = Block::new(
            0,
            Vec::new(),
            String::from("0"),
        );

        println!("Minerando o bloco gênesis...");
        while !genesis_block.calculate_hash().starts_with(&difficulty){
            genesis_block.nonce += 1;
        }

        genesis_block.hash = genesis_block.calculate_hash();
        chain.insert(genesis_block.hash.clone(), genesis_block.clone());

        let last = genesis_block.hash.clone();

        println!("Bloco Gênese minerado e adicionado. Hash: {}", last);

        Blockchain {
            chain,
            last,
            difficulty,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>){
        let previous_block = self.chain.get(&self.last).expect("Bloco anterior não encontrado");
        let new_index = previous_block.index + 1;
        let new_previous_hash = previous_block.hash.clone();

        let mut new_block = Block::new(
            new_index,
            transactions,
            new_previous_hash.clone(),
        );

        println!("\nIniciando mineração do bloco #{}...", new_block.index);

        while !new_block.calculate_hash().starts_with(&self.difficulty) {
            new_block.nonce += 1;
        }

        new_block.hash = new_block.calculate_hash();
        println!("Bloco #{} minerado! Nonce: {}, Hash: {}",
                 new_block.index, new_block.nonce, new_block.hash);

        self.chain.insert(new_block.hash.clone(), new_block.clone());

        self.last = new_block.hash.clone();
    }

    pub fn is_valid(&mut self) -> bool{
        let current_hash = self.last.clone();

        let current_block_option = self.chain.get(&current_hash);

        if current_block_option.is_none() {
            return false;
        }

        let mut current_block = current_block_option.unwrap();

        loop {
            println!("Validando bloco #{}...", current_block_option.unwrap().index);

            let calculated_hash = current_block.calculate_hash();
            if calculated_hash != current_block.hash {
                println!("Bloco #{} inválido! Hash calculado: {}, Hash armazenado: {}",
                         current_block.index, calculated_hash, current_block.hash);
                return false;
            }

            if !calculated_hash.starts_with(&self.difficulty) {
                println!("Bloco #{} inválido! Hash não atende ao critério de dificuldade: {}",
                         current_block.index, self.difficulty);
                return false;
            }

            if current_block.previous_hash != current_hash {
                println!("Bloco #{} inválido! Hash anterior não corresponde: {}",
                         current_block.index, current_block.previous_hash);
                return false;
            }

            for tx in &current_block.transactions {
                if !Transaction::verify_signature(&tx){
                    println!("Transação inválida no bloco #{}: {:?}", current_block.index, tx);
                    return false;
                }
            }

            if current_block.index == 0 {
                break;
            }

            let previous_block_option = self.chain.get(&current_block.previous_hash);

            if previous_block_option.is_none() {
                println!("ERRO DE VALIDAÇÃO: Bloco anterior (hash: {}) não encontrado no HashMap para o Bloco #{}!",
                         current_block.previous_hash, current_block.index);
                return false; 
            }

            let previous_block = previous_block_option.unwrap();

            if current_block.previous_hash != previous_block.hash {
                 println!("ERRO DE VALIDAÇÃO: Bloco #{} tem previous_hash ({}) incorreto! Esperava-se {}",
                          current_block.index, current_block.previous_hash, previous_block.hash);
                 return false;
            }

            current_block = previous_block;
        }

        println!("Todos os blocos e transações validados com sucesso! A blockchain é válida.");
        true


    }

    pub fn get_current_balances(&self) -> HashMap<String, u64> {
        let mut balances: HashMap<String, u64> = HashMap::new();

        let mut ordered_blocks: Vec<&Block> = self.chain.values().collect();
        ordered_blocks.sort_by_key(|b| b.index);

        for block in ordered_blocks {

            for transaction in &block.transactions {
                if !transaction.verify_signature(){
                    continue;;
                }

                let sender_key_str = serde_json::to_string(&transaction.sender_public_key).expect("Falha ao serializar chave pública do remetente para String.");

                let recipient_key_str = serde_json::to_string(&transaction.recipient_public_key).expect("Falha ao serializar chave pública do destinatário para String.");
                
                *balances.entry(sender_key_str).or_insert(0) = balances.get(&sender_key_str).unwrap_or(&0).saturating_sub(transaction.value);

                *balances.entry(recipient_key_str).or_insert(0) = balances.get(&recipient_key_str).unwrap_or(&0).saturating_add(transaction.value);
           
            }
        }

        balances
    }

}