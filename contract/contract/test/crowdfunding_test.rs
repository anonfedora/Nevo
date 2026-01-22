#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, BytesN, Env, String,
};

use crate::{
    base::{errors::CrowdfundingError, types::PoolState},
    crowdfunding::{CrowdfundingContract, CrowdfundingContractClient},
};

fn create_test_campaign_id(env: &Env, seed: u8) -> BytesN<32> {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    BytesN::from_array(env, &bytes)
}

#[test]
fn test_create_campaign() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 1);
    let title = String::from_str(&env, "Save the Whales");
    let goal = 1_000_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    client.create_campaign(&campaign_id, &title, &creator, &goal, &deadline);
}

#[test]
fn test_get_campaign() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 2);
    let title = String::from_str(&env, "Build a School");
    let goal = 500_000i128;
    let deadline = env.ledger().timestamp() + 172800;

    client.create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    let campaign = client.get_campaign(&campaign_id);

    assert_eq!(campaign.id, campaign_id);
    assert_eq!(campaign.title, title);
    assert_eq!(campaign.creator, creator);
    assert_eq!(campaign.goal, goal);
    assert_eq!(campaign.deadline, deadline);
}

#[test]
fn test_get_nonexistent_campaign() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let campaign_id = create_test_campaign_id(&env, 99);

    let result = client.try_get_campaign(&campaign_id);

    assert_eq!(result, Err(Ok(CrowdfundingError::CampaignNotFound)));
}

#[test]
fn test_create_campaign_with_empty_title() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 3);
    let title = String::from_str(&env, "");
    let goal = 100_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    let result = client.try_create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidTitle)));
}

#[test]
fn test_create_campaign_with_zero_goal() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 4);
    let title = String::from_str(&env, "Zero Goal Campaign");
    let goal = 0i128;
    let deadline = env.ledger().timestamp() + 86400;

    let result = client.try_create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidGoal)));
}

#[test]
fn test_create_campaign_with_negative_goal() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 5);
    let title = String::from_str(&env, "Negative Goal Campaign");
    let goal = -100i128;
    let deadline = env.ledger().timestamp() + 86400;

    let result = client.try_create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidGoal)));
}

#[test]
fn test_create_campaign_with_past_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000);

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 6);
    let title = String::from_str(&env, "Past Deadline Campaign");
    let goal = 100_000i128;
    let deadline = 500;

    let result = client.try_create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidDeadline)));
}

#[test]
fn test_create_duplicate_campaign() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let campaign_id = create_test_campaign_id(&env, 7);
    let title = String::from_str(&env, "Duplicate Campaign");
    let goal = 100_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    client.create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    let result2 = client.try_create_campaign(&campaign_id, &title, &creator, &goal, &deadline);

    assert_eq!(result2, Err(Ok(CrowdfundingError::CampaignAlreadyExists)));
}

#[test]
fn test_multiple_campaigns() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator1 = Address::generate(&env);
    let creator2 = Address::generate(&env);

    let campaign_id_1 = create_test_campaign_id(&env, 8);
    let campaign_id_2 = create_test_campaign_id(&env, 9);

    let title1 = String::from_str(&env, "Campaign One");
    let title2 = String::from_str(&env, "Campaign Two");

    let goal1 = 100_000i128;
    let goal2 = 200_000i128;

    let deadline1 = env.ledger().timestamp() + 86400;
    let deadline2 = env.ledger().timestamp() + 172800;

    client.create_campaign(&campaign_id_1, &title1, &creator1, &goal1, &deadline1);
    client.create_campaign(&campaign_id_2, &title2, &creator2, &goal2, &deadline2);

    let campaign1 = client.get_campaign(&campaign_id_1);
    let campaign2 = client.get_campaign(&campaign_id_2);

    assert_eq!(campaign1.title, title1);
    assert_eq!(campaign1.goal, goal1);

    assert_eq!(campaign2.title, title2);
    assert_eq!(campaign2.goal, goal2);
}

// Pool Storage Tests

#[test]
fn test_save_pool() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let name = String::from_str(&env, "Education Fund");
    let description = String::from_str(&env, "Fund for educational supplies");
    let target_amount = 10_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    let pool_id = client.save_pool(&name, &description, &creator, &target_amount, &deadline);

    assert_eq!(pool_id, 1);
}

#[test]
fn test_save_pool_validation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);

    // Test empty name
    let empty_name = String::from_str(&env, "");
    let description = String::from_str(&env, "Description");
    let target_amount = 10_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    let result = client.try_save_pool(
        &empty_name,
        &description,
        &creator,
        &target_amount,
        &deadline,
    );
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidPoolName)));

    // Test invalid target amount
    let name = String::from_str(&env, "Test Pool");
    let result = client.try_save_pool(&name, &description, &creator, &0i128, &deadline);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidPoolTarget)));

    // Test invalid deadline
    let past_deadline = 0; // Use 0 as a past timestamp since ledger starts at 0
    let result = client.try_save_pool(
        &name,
        &description,
        &creator,
        &target_amount,
        &past_deadline,
    );
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidPoolDeadline)));
}

#[test]
fn test_get_pool() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let name = String::from_str(&env, "Medical Fund");
    let description = String::from_str(&env, "Fund for medical expenses");
    let target_amount = 5_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    let pool_id = client.save_pool(&name, &description, &creator, &target_amount, &deadline);

    let pool = client.get_pool(&pool_id).unwrap();

    assert_eq!(pool.id, pool_id);
    assert_eq!(pool.name, name);
    assert_eq!(pool.description, description);
    assert_eq!(pool.creator, creator);
    assert_eq!(pool.target_amount, target_amount);
    assert_eq!(pool.deadline, deadline);
    assert!(pool.created_at <= env.ledger().timestamp()); // created_at should be <= current time
}

#[test]
fn test_get_nonexistent_pool() {
    let env = Env::default();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let pool = client.get_pool(&999);
    assert!(pool.is_none());
}

#[test]
fn test_update_pool_state() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let name = String::from_str(&env, "Charity Fund");
    let description = String::from_str(&env, "Fund for charity");
    let target_amount = 15_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    let pool_id = client.save_pool(&name, &description, &creator, &target_amount, &deadline);

    // Update state to Paused
    client.update_pool_state(&pool_id, &PoolState::Paused);

    // Update state to Completed
    client.update_pool_state(&pool_id, &PoolState::Completed);
}

#[test]
fn test_update_pool_state_nonexistent() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let result = client.try_update_pool_state(&999, &PoolState::Paused);
    assert_eq!(result, Err(Ok(CrowdfundingError::PoolNotFound)));
}

#[test]
fn test_update_pool_state_invalid_transition() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let name = String::from_str(&env, "Test Fund");
    let description = String::from_str(&env, "Test fund");
    let target_amount = 10_000i128;
    let deadline = env.ledger().timestamp() + 86400;

    let pool_id = client.save_pool(&name, &description, &creator, &target_amount, &deadline);

    // First complete the pool
    client.update_pool_state(&pool_id, &PoolState::Completed);

    // Try to change state from completed - should fail
    let result = client.try_update_pool_state(&pool_id, &PoolState::Active);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidPoolState)));

    let result = client.try_update_pool_state(&pool_id, &PoolState::Paused);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidPoolState)));
}

#[test]
fn test_multiple_pools() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let creator1 = Address::generate(&env);
    let creator2 = Address::generate(&env);

    // Create first pool
    let name1 = String::from_str(&env, "Pool One");
    let description1 = String::from_str(&env, "First pool");
    let target1 = 10_000i128;
    let deadline1 = env.ledger().timestamp() + 86400;
    let pool_id1 = client.save_pool(&name1, &description1, &creator1, &target1, &deadline1);

    // Create second pool
    let name2 = String::from_str(&env, "Pool Two");
    let description2 = String::from_str(&env, "Second pool");
    let target2 = 20_000i128;
    let deadline2 = env.ledger().timestamp() + 172800;
    let pool_id2 = client.save_pool(&name2, &description2, &creator2, &target2, &deadline2);

    assert_eq!(pool_id1, 1);
    assert_eq!(pool_id2, 2);

    // Verify both pools
    let pool1 = client.get_pool(&pool_id1).unwrap();
    let pool2 = client.get_pool(&pool_id2).unwrap();

    assert_eq!(pool1.name, name1);
    assert_eq!(pool1.target_amount, target1);

    assert_eq!(pool2.name, name2);
    assert_eq!(pool2.target_amount, target2);

    // Update different states
    client.update_pool_state(&pool_id1, &PoolState::Paused);
    client.update_pool_state(&pool_id2, &PoolState::Active);
}
