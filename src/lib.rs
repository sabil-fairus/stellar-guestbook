#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct GuestEntry {
    pub name: String,
    pub message: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    GuestBookEntries,
}

#[contract]
pub struct GuestBookContract;

#[contractimpl]
impl GuestBookContract {
    
    pub fn get_entries(env: Env) -> Vec<GuestEntry> {
        let key = DataKey::GuestBookEntries;
        env.storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn add_entry(env: Env, name: String, message: String) -> String {
        let key = DataKey::GuestBookEntries;
        
        let mut entries: Vec<GuestEntry> = env.storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        
        let timestamp = env.ledger().timestamp();
        
        let new_entry = GuestEntry {
            name,
            message,
            timestamp,
        };
        
        entries.push_back(new_entry);
        env.storage().instance().set(&key, &entries);
        
        String::from_str(&env, "✅ Thank you for signing the guest book!")
    }

    // HAPUS entri berdasarkan INDEX (0 = entri pertama)
    pub fn delete_entry(env: Env, index: u32) -> String {
        let key = DataKey::GuestBookEntries;
        
        let mut entries: Vec<GuestEntry> = env.storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        
        if index >= entries.len() {
            return String::from_str(&env, "❌ Error: Invalid index!");
        }
        
        entries.remove(index);
        env.storage().instance().set(&key, &entries);
        
        String::from_str(&env, "✅ Entry deleted successfully!")
    }

    // HAPUS SEMUA entri
    pub fn clear_all_entries(env: Env) -> String {
        let key = DataKey::GuestBookEntries;
        let empty_entries: Vec<GuestEntry> = Vec::new(&env);
        
        env.storage().instance().set(&key, &empty_entries);
        
        String::from_str(&env, "✅ All entries cleared!")
    }
}