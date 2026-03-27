#![no_std]

use soroban_sdk::{
    contract, contractimpl, Env, Symbol, symbol_short, Address, String
};

// Storage keys
const OWNER: Symbol = symbol_short!("OWNER");

#[contract]
pub struct SkinNFT;

#[contractimpl]
impl SkinNFT {

    // Mint a new gaming skin NFT
    pub fn mint(env: Env, skin_id: u32, to: Address, metadata: String) {
        // Store owner
        env.storage().instance().set(&(OWNER, skin_id), &to);

        // Store metadata (like skin name, rarity, etc.)
        env.storage().instance().set(&("META", skin_id), &metadata);
    }

    // Get owner of a skin
    pub fn get_owner(env: Env, skin_id: u32) -> Address {
        env.storage()
            .instance()
            .get(&(OWNER, skin_id))
            .unwrap()
    }

    // Transfer skin NFT
    pub fn transfer(env: Env, skin_id: u32, from: Address, to: Address) {
        let owner: Address = env.storage().instance().get(&(OWNER, skin_id)).unwrap();

        // Ensure only owner can transfer
        if owner != from {
            panic!("Not the owner");
        }

        env.storage().instance().set(&(OWNER, skin_id), &to);
    }

    // Get metadata of skin
    pub fn get_metadata(env: Env, skin_id: u32) -> String {
        env.storage()
            .instance()
            .get(&("META", skin_id))
            .unwrap()
    }
}