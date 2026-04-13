#![no_std]
// Updated for Veloce-rs
use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short};

#[contract]
pub struct VeloceProtocol;

#[contractimpl]
impl VeloceProtocol {
    pub fn init_stream(e: Env, sender: Address, receiver: Address, velocity: i128) {
        sender.require_auth();
        // Using "velocity" instead of "rate" for the brand flavor
        e.storage().instance().set(&symbol_short!("vel"), &velocity);
        e.storage().instance().set(&symbol_short!("rx"), &receiver);
        e.storage().instance().set(&symbol_short!("start"), &e.ledger().timestamp());
    }

    pub fn current_flow(e: Env) -> i128 {
        let start: u64 = e.storage().instance().get(&symbol_short!("start")).unwrap();
        let vel: i128 = e.storage().instance().get(&symbol_short!("vel")).unwrap();
        
        let elapsed = (e.ledger().timestamp() - start) as i128;
        elapsed * vel
    }
}
