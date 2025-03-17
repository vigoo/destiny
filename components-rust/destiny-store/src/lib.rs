#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::destiny::store_exports::destiny_store_api::*;
use std::cell::RefCell;

struct State {
    total: u64,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        total: 0,
    });
}

struct Component;

impl Guest for Component {
    fn add_destination(name: String, destination: UserDefinedDestination) {
        todo!()
    }

    fn get_currency() -> Currency {
        todo!()
    }

    fn get_destination(name: String) -> Destination {
        todo!()
    }

    fn get_destinations() -> Vec<Destination> {
        todo!()
    }

    fn get_home_location() -> String {
        todo!()
    }

    fn get_ordered_destinations(preferences: Preferences) -> Vec<Destination> {
        todo!()
    }

    fn remove_destination(name: String) {
        todo!()
    }

    fn set_currency(currency: Currency) {
        todo!()
    }

    fn set_home_location(location: String) {
        todo!()
    }

    fn update_destination(name: String, destination: UserDefinedDestination) {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
