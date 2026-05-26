#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

mod tests {
    use super::*;

    /// Test 1 (Happy Path): Complete MVP flow - create tab, make payment, verify reduction
    #[test]
    fn test_full_payment_flow() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, SariSplitContract);
        let client = SariSplitContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let customer = Address::generate(&env);
        
        // Initialize and create tab
        client.initialize(&owner);
        client.create_tab(&customer, &1000);
        
        // Customer pays 400
        client.pay_tab(&customer, &400);
        
        // Verify balance reduced correctly
        let tab = client.get_tab(&customer);
        assert_eq!(tab.balance_due, 600);
        assert_eq!(tab.total_paid, 400);
    }

    /// Test 2 (Edge Case): Payment exceeding balance should fail
    #[test]
    #[should_panic(expected = "Payment exceeds balance due")]
    fn test_overpayment_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, SariSplitContract);
        let client = SariSplitContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let customer = Address::generate(&env);
        
        client.initialize(&owner);
        client.create_tab(&customer, &500);
        
        // Attempt to pay more than owed - should panic
        client.pay_tab(&customer, &600);
    }

    /// Test 3 (State Verification): Verify storage state after multiple operations
    #[test]
    fn test_state_after_multiple_payments() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, SariSplitContract);
        let client = SariSplitContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let customer = Address::generate(&env);
        
        client.initialize(&owner);
        client.create_tab(&customer, &1000);
        
        // Multiple partial payments
        client.pay_tab(&customer, &200);
        client.pay_tab(&customer, &300);
        client.pay_tab(&customer, &100);
        
        let tab = client.get_tab(&customer);
        assert_eq!(tab.balance_due, 400);  // 1000 - 200 - 300 - 100
        assert_eq!(tab.total_paid, 600);   // 200 + 300 + 100
    }

    /// Test 4: Adding to existing tab works correctly
    #[test]
    fn test_add_to_tab() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, SariSplitContract);
        let client = SariSplitContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let customer = Address::generate(&env);
        
        client.initialize(&owner);
        client.create_tab(&customer, &500);
        client.add_to_tab(&customer, &250);
        
        let tab = client.get_tab(&customer);
        assert_eq!(tab.balance_due, 750);
    }

    /// Test 5: Duplicate tab creation should fail
    #[test]
    #[should_panic(expected = "Tab already exists")]
    fn test_duplicate_tab_fails() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, SariSplitContract);
        let client = SariSplitContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let customer = Address::generate(&env);
        
        client.initialize(&owner);
        client.create_tab(&customer, &500);
        client.create_tab(&customer, &300); // Should panic
    }
}
