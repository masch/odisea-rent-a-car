use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn car_returned(env: &Env, owner: &Address) {
    let topics = (Symbol::new(env, "car_retuned"), owner.clone());

    env.events().publish(topics, ());
}
