#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::destiny::user_exports::destiny_user_api::*;
use std::cell::RefCell;
use std::collections::HashSet;
use golem_rust::bindings::golem::api::host::resolve_worker_id;

struct State {
    stores: HashSet<StoreName>
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
        STATE.with_borrow(|state| {
            state.stores.iter().cloned().collect()
        })
    }
}

fn initialize_store_worker(name: &StoreName) {
    // let worker_id = resolve_worker_id("destiny:store", &store_worker_name(owner_email, name));
    todo!()
}

bindings::export!(Component with_types_in bindings);
