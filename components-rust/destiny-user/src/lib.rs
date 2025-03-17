#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::destiny::accounts_client::destiny_accounts_client::DestinyAccountsApi;
use crate::bindings::destiny::store_client::destiny_store_client::{DestinyStoreApi, GolemRpcUri};
use crate::bindings::destiny::store_exports::destiny_store_api::User;
use crate::bindings::exports::destiny::user_exports::destiny_user_api::*;
use destiny_model::store_worker_name;
use golem_rust::bindings::golem::api::host::{resolve_worker_id, worker_uri};
use std::cell::RefCell;
use std::env;

struct State {
    stores: Vec<(User, StoreName)>,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        stores: Vec::new()
    });
}

struct Component;

impl Guest for Component {
    fn create_store(name: StoreName) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            if state.stores.iter().any(|(_, n)| n == &name) {
                Err(Error::AlreadyExists)
            } else {
                initialize_store_worker(&name);
                state.stores.push((self_user(), name));
                Ok(())
            }
        })
    }

    fn get_user_name(
        email: String,
    ) -> bindings::exports::destiny::user_exports::destiny_user_api::User {
        let worker_id = resolve_worker_id("destiny:accounts", "__accounts_proxy")
            .expect("Failed to resolve worker_id");
        let target_uri = worker_uri(&worker_id);
        let remote_api = DestinyAccountsApi::new(&GolemRpcUri {
            value: target_uri.value,
        });
        remote_api.blocking_get_user_name(&email)
    }

    fn stores() -> Vec<(User, StoreName)> {
        STATE.with_borrow(|state| state.stores.iter().cloned().collect())
    }
}

fn initialize_store_worker(name: &StoreName) {
    let owner_user: User = self_user();
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

fn self_user() -> User {
    env::var("GOLEM_WORKER_NAME").expect("GOLEM_WORKER_NAME is not available")
}

bindings::export!(Component with_types_in bindings);
