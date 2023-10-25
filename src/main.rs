use solana_sdk::{pubkey::Pubkey, instruction::{Instruction, AccountMeta}, signature::Keypair, signer::Signer};

fn main() {
    println!("Hello from Solana Rust SDK!");
    let keypair = Keypair::new();
    let my_pubkey = keypair.try_pubkey().unwrap();
    let random_pubkey = Pubkey::new_unique();
    let system_program_id = Pubkey::default();
    let instruction_data = &[2, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0];
    let instruction_accounts = vec![AccountMeta { pubkey: my_pubkey, is_signer: true, is_writable: true }, AccountMeta { pubkey: random_pubkey, is_signer: false, is_writable: true }];

    Instruction::new_with_bytes(system_program_id, instruction_data, instruction_accounts);
}
