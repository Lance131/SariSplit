#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol,
};

// Storage keys for the contract
const OWNER: Symbol = symbol_short!("OWNER");
const TABS: Symbol = symbol_short!("TABS");

/// Represents a customer's tab (running credit balance)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tab {
    pub customer: Address,
    pub balance_due: i128,      // Amount customer owes (in stroops/smallest unit)
    pub total_paid: i128,       // Total amount paid over time
    pub created_at: u64,        // Ledger timestamp when tab was created
}

#[contract]
pub struct SariSplitContract;

#[contractimpl]
impl SariSplitContract {
    /// Initialize the contract with the store owner's address
    /// Called once when deploying the contract
    pub fn initialize(env: Env, owner: Address) {
        // Ensure contract hasn't been initialized already
        if env.storage().instance().has(&OWNER) {
            panic!("Contract already initialized");
        }
        
        // Store the owner and create empty tabs map
        env.storage().instance().set(&OWNER, &owner);
        let tabs: Map<Address, Tab> = Map::new(&env);
        env.storage().instance().set(&TABS, &tabs);
    }

    /// Store owner creates a new tab for a customer with an initial balance due
    /// Only the owner can create tabs
    pub fn create_tab(env: Env, customer: Address, initial_balance: i128) {
        let owner: Address = env.storage().instance().get(&OWNER).unwrap();
        owner.require_auth(); // Only owner can create tabs
        
        if initial_balance <= 0 {
            panic!("Initial balance must be positive");
        }

        let mut tabs: Map<Address, Tab> = env.storage().instance().get(&TABS).unwrap();
        
        // Prevent duplicate tabs
        if tabs.contains_key(customer.clone()) {
            panic!("Tab already exists for this customer");
        }

        let tab = Tab {
            customer: customer.clone(),
            balance_due: initial_balance,
            total_paid: 0,
            created_at: env.ledger().timestamp(),
        };

        tabs.set(customer, tab);
        env.storage().instance().set(&TABS, &tabs);
    }

    /// Customer pays down their tab balance
    /// This records the payment; actual USDC transfer happens off-chain or via separate token contract
    pub fn pay_tab(env: Env, customer: Address, amount: i128) {
        customer.require_auth(); // Customer must authorize their own payment
        
        if amount <= 0 {
            panic!("Payment amount must be positive");
        }

        let mut tabs: Map<Address, Tab> = env.storage().instance().get(&TABS).unwrap();
        
        let mut tab = tabs.get(customer.clone()).expect("No tab found for customer");
        
        // Prevent overpayment
        if amount > tab.balance_due {
            panic!("Payment exceeds balance due");
        }

        tab.balance_due -= amount;
        tab.total_paid += amount;
        
        tabs.set(customer, tab);
        env.storage().instance().set(&TABS, &tabs);
    }

    /// Owner adds more credit to an existing customer's tab
    pub fn add_to_tab(env: Env, customer: Address, amount: i128) {
        let owner: Address = env.storage().instance().get(&OWNER).unwrap();
        owner.require_auth();
        
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let mut tabs: Map<Address, Tab> = env.storage().instance().get(&TABS).unwrap();
        let mut tab = tabs.get(customer.clone()).expect("No tab found for customer");
        
        tab.balance_due += amount;
        
        tabs.set(customer, tab);
        env.storage().instance().set(&TABS, &tabs);
    }

    /// View a customer's current tab status
    pub fn get_tab(env: Env, customer: Address) -> Tab {
        let tabs: Map<Address, Tab> = env.storage().instance().get(&TABS).unwrap();
        tabs.get(customer).expect("No tab found for customer")
    }

    /// Get the store owner's address
    pub fn get_owner(env: Env) -> Address {
        env.storage().instance().get(&OWNER).unwrap()
    }
}
