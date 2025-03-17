#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::destiny::store_client::destiny_store_client::{DestinyStoreApi, GolemRpcUri};
use crate::bindings::destiny::store_exports::destiny_store_api::User;
use crate::bindings::exports::destiny::user_exports::destiny_user_api::*;
use destiny_model::store_worker_name;
use golem_rust::bindings::golem::api::host::{resolve_worker_id, worker_uri};
use std::cell::RefCell;
use std::collections::HashSet;
use std::env;

struct State {
    stores: HashSet<StoreName>,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        stores: HashSet::new()
    });
}

struct Component;

impl Guest for Component {
    fn create_store(name: StoreName) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            if state.stores.contains(&name) {
                Err(Error::AlreadyExists)
            } else {
                initialize_store_worker(&name);
                state.stores.insert(name);
                Ok(())
            }
        })
    }

    fn stores() -> Vec<StoreName> {
        STATE.with_borrow(|state| state.stores.iter().cloned().collect())
    }
}

fn initialize_store_worker(name: &StoreName) {
    let owner_user: User =
        env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME is not available");
    let worker_id = resolve_worker_id("destiny:store", &store_worker_name(&owner_user, name))
        .expect("Failed to resolve store worker ID");
    let target_uri = worker_uri(&worker_id);

    let remote_api = DestinyStoreApi::new(&GolemRpcUri {
        value: target_uri.value,
    });
    let success = remote_api.blocking_initialize(&owner_user);
    if !success {
        panic!("Failed to initialize store worker - it already existed");
    }
}

bindings::export!(Component with_types_in bindings);
