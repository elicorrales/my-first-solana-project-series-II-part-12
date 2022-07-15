use my_first_on_chain_bin_plus_lib::process_instruction;
use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    stake_history::Epoch,
    msg
};
use std::env;

fn main() {

    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    //println!("\nCmd-line args: {:?}\n", args);
    //println!("\nCmd-line args len: {:?}\n", args.len());

    let mut instruction_data  = "not set".as_bytes();
    if args.len() > 1 {
        instruction_data = &args[1].as_bytes();
    }

    let program_id        = Pubkey::default();
    let accounts          = vec![]; 
    //let instruction_data  = vec![0,3,3,5,32,6,23];

    process_instruction(&program_id, &accounts, &instruction_data);
}


