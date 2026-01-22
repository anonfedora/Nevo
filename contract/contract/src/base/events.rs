use soroban_sdk::{Address, BytesN, Env, String, Symbol};

use crate::base::types::PoolState;

pub fn campaign_created(
    env: &Env,
    id: BytesN<32>,
    title: String,
    creator: Address,
    goal: i128,
    deadline: u64,
) {
    let topics = (Symbol::new(env, "campaign_created"), id, creator);
    env.events().publish(topics, (title, goal, deadline));
}

pub fn pool_created(
    env: &Env,
    pool_id: u64,
    name: String,
    description: String,
    creator: Address,
    target_amount: i128,
    deadline: u64,
) {
    let topics = (Symbol::new(env, "pool_created"), pool_id, creator);
    env.events()
        .publish(topics, (name, description, target_amount, deadline));
}

pub fn pool_state_updated(env: &Env, pool_id: u64, new_state: PoolState) {
    let topics = (Symbol::new(env, "pool_state_updated"), pool_id);
    env.events().publish(topics, new_state);
}
