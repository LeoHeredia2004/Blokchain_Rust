mod block;
mod sha;
mod blockchain;
mod transaction;
mod rsa;
mod wallet;


fn main() {
    //------------------- Inicializando Blockchain ----------------------------
    println!("--- Iniciando a Simulação da Blockchain com RSA ---");

    // 1. Criar algumas carteiras (pares de chaves RSA)
    println!("\n1. Gerando carteiras...");
    let wallet_alice = wallet::Wallet::new(String::from("Alice"));
    let wallet_bob = wallet::Wallet::new(String::from("Bob"));
    let wallet_charlie = wallet::Wallet::new(String::from("Charlie"));

    println!("   Carteiras geradas:");
    println!("   Alice (E,N): {:?}", wallet_alice.public_key);
    println!("   Bob (E,N):   {:?}", wallet_bob.public_key);
    println!("   Charlie (E,N): {:?}", wallet_charlie.public_key);


    // 2. Inicializar a Blockchain (isso inclui a mineração do Bloco Gênese)
    println!("\n2. Inicializando a Blockchain...");
    let mut my_blockchain = blockchain::Blockchain::new();
    println!("   Blockchain inicializada. Dificuldade: {}", my_blockchain.difficulty);


    // 3. Criar e adicionar Blocos com Transações Assinadas

    // --- Bloco 1 ---
    let mut transactions_block_1: Vec<transaction::Transaction> = Vec::new();
    let mut tx1 = transaction::Transaction::new(
        wallet_alice.public_key.clone(),
        wallet_bob.public_key.clone(), // Agora passa a chave pública de Bob
        100,
    );
    tx1.sign(&wallet_alice.private_key);
    transactions_block_1.push(tx1);

    let mut tx2 = transaction::Transaction::new(
        wallet_charlie.public_key.clone(),
        wallet_bob.public_key.clone(), // Agora passa a chave pública de Bob
        50,
    );
    tx2.sign(&wallet_charlie.private_key);
    transactions_block_1.push(tx2);
    my_blockchain.add_block(transactions_block_1);

    // ... (Continue com o Bloco 2 e 3 de forma similar) ...

    // --- Nova parte: Calcular e Imprimir Saldos ---
    println!("\n--- 6. Saldos Atuais ---");
    let balances = my_blockchain.get_current_balances();
    for (address_json, balance) in balances {
        println!("   Endereço (JSON): {} -> Saldo: {}", address_json, balance);
    }
    
    println!("\n--- Simulação Concluída ---");


}