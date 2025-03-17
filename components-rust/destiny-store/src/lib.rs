#[allow(static_mut_refs)]
mod bindings;
mod logic;

use crate::bindings::exports::destiny::store_exports::destiny_store_api::*;
use crate::logic::{DestinationExtensions, PreferencesExtensions};
use std::cell::RefCell;
use std::collections::HashMap;

struct State {
    currency: Currency,
    home_location: String,
    destinations: HashMap<String, Destination>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        currency: "HUF".to_string(),
        home_location: "Kosd, Hungary".to_string(),
        destinations: HashMap::new()
    });
}

struct Component;

impl Guest for Component {
    fn add_destination(
        name: DestinationName,
        destination: UserDefinedDestination,
    ) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            if state.destinations.contains_key(&name) {
                Err(Error::AlreadyExists(name))
            } else {
                state.destinations.insert(
                    name.clone(),
                    Destination {
                        name,
                        user_defined_destination: destination,
                    },
                );
                Ok(())
            }
        })
    }

    fn get_currency() -> Currency {
        STATE.with_borrow(|state| state.currency.clone())
    }

    fn get_destination(name: DestinationName) -> Option<Destination> {
        STATE.with_borrow(|state| state.destinations.get(&name).cloned())
    }

    fn get_destinations() -> Vec<Destination> {
        STATE.with_borrow(|state| state.destinations.values().cloned().collect())
    }

    fn get_home_location() -> String {
        STATE.with_borrow(|state| state.home_location.clone())
    }

    fn get_ordered_destinations(preferences: Preferences) -> Vec<Destination> {
        STATE.with_borrow(|state| {
            let mut destinations: Vec<_> = state
                .destinations
                .values()
                .filter(|destination| preferences.filter(destination))
                .cloned()
                .collect();
            destinations.sort_by_key(|destination| destination.rating(&preferences.month));
            destinations
        })
    }

    fn remove_destination(name: DestinationName) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            if let Some(_) = state.destinations.remove(&name) {
                Ok(())
            } else {
                Err(Error::NotFound(name))
            }
        })
    }

    fn set_currency(currency: Currency) {
        STATE.with_borrow_mut(|state| {
            state.currency = currency;
        })
    }

    fn set_home_location(location: String) {
        STATE.with_borrow_mut(|state| state.home_location = location)
    }

    fn update_destination(
        name: DestinationName,
        destination: UserDefinedDestination,
    ) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            if let Some(dest) = state.destinations.get_mut(&name) {
                dest.user_defined_destination = destination;
                Ok(())
            } else {
                Err(Error::NotFound(name))
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);
