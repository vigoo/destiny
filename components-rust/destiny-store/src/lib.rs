#[allow(static_mut_refs)]
mod bindings;
mod logic;

use crate::bindings::exports::destiny::store_exports::destiny_store_api::*;
use crate::logic::{DestinationExtensions, PreferencesExtensions};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

struct State {
    owner: Option<User>,
    shared_with: HashSet<User>,
    currency: Currency,
    home_location: String,
    destinations: HashMap<String, Destination>,
}

impl State {
    pub fn authorize(&self, user: &User) -> Result<(), Error> {
        if let Some(owner) = &self.owner {
            if owner == user {
                Ok(())
            } else {
                if self.shared_with.contains(user) {
                    Ok(())
                } else {
                    Err(Error::AccessDenied)
                }
            }
        } else {
            Err(Error::NotInitialized)
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        owner: None,
        shared_with: HashSet::new(),
        currency: "HUF".to_string(),
        home_location: "Kosd, Hungary".to_string(),
        destinations: HashMap::new()
    });
}

struct StoreAccess {
    user: User,
}

impl GuestStore for StoreAccess {
    fn new(user: User) -> Self {
        Self { user }
    }

    fn set_currency(&self, currency: Currency) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            state.authorize(&self.user)?;

            state.currency = currency;
            Ok(())
        })
    }

    fn get_currency(&self) -> Result<Currency, Error> {
        STATE.with_borrow(|state| {
            state.authorize(&self.user)?;

            Ok(state.currency.clone())
        })
    }

    fn set_home_location(&self, location: String) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            state.authorize(&self.user)?;

            state.home_location = location;
            Ok(())
        })
    }

    fn get_home_location(&self) -> Result<String, Error> {
        STATE.with_borrow(|state| {
            state.authorize(&self.user)?;

            Ok(state.home_location.clone())
        })
    }

    fn add_destination(
        &self,
        name: DestinationName,
        destination: UserDefinedDestination,
    ) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            state.authorize(&self.user)?;

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

    fn update_destination(
        &self,
        name: DestinationName,
        destination: UserDefinedDestination,
    ) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            state.authorize(&self.user)?;

            if let Some(dest) = state.destinations.get_mut(&name) {
                dest.user_defined_destination = destination;
                Ok(())
            } else {
                Err(Error::NotFound(name))
            }
        })
    }

    fn get_destination(&self, name: DestinationName) -> Result<Option<Destination>, Error> {
        STATE.with_borrow(|state| {
            state.authorize(&self.user)?;

            Ok(state.destinations.get(&name).cloned())
        })
    }

    fn get_destinations(&self) -> Result<Vec<Destination>, Error> {
        STATE.with_borrow(|state| {
            state.authorize(&self.user)?;

            Ok(state.destinations.values().cloned().collect())
        })
    }

    fn remove_destination(&self, name: DestinationName) -> Result<(), Error> {
        STATE.with_borrow_mut(|state| {
            state.authorize(&self.user)?;

            if let Some(_) = state.destinations.remove(&name) {
                Ok(())
            } else {
                Err(Error::NotFound(name))
            }
        })
    }

    fn get_ordered_destinations(
        &self,
        preferences: Preferences,
    ) -> Result<Vec<Destination>, Error> {
        STATE.with_borrow(|state| {
            state.authorize(&self.user)?;

            let mut destinations: Vec<_> = state
                .destinations
                .values()
                .filter(|destination| preferences.filter(destination))
                .cloned()
                .collect();
            destinations.sort_by_key(|destination| destination.rating(&preferences.month));
            Ok(destinations)
        })
    }
}

struct Component;

impl Guest for Component {
    type Store = StoreAccess;

    fn initialize(user: User) -> bool {
        STATE.with_borrow_mut(|state| {
            if state.owner.is_none() {
                state.owner = Some(user);
                true
            } else {
                false
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);
