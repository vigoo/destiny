#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::destiny::accounts_exports::destiny_accounts_api::*;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;

struct State {
    accounts: HashMap<String, String>,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        accounts: HashMap::new()
    });
}

struct Component;

impl Guest for Component {
    fn get_user_name(email: String) -> User {
        STATE.with_borrow_mut(|state| {
            state
                .accounts
                .entry(email)
                .or_insert(Uuid::new_v4().to_string())
                .clone()
        })
    }
}

bindings::export!(Component with_types_in bindings);
