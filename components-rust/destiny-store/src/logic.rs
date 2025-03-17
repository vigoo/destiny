use crate::bindings::exports::destiny::store_exports::destiny_store_api::{
    Destination, Month, Preferences, Rating, TravelBy, TravelLength,
};

pub trait PreferencesExtensions {
    fn filter(&self, destination: &Destination) -> bool;
}

impl PreferencesExtensions for Preferences {
    fn filter(&self, destination: &Destination) -> bool {
        filter_by_travel_length(&self.lengths, &destination.user_defined_destination.lengths)
            && filter_by_vehicle(
                &self.travel_by,
                &destination.user_defined_destination.travel_by,
            )
    }
}

fn filter_by_travel_length(
    preferred: &Option<TravelLength>,
    specified: &Option<TravelLength>,
) -> bool {
    if let Some(allowed_lengths) = preferred {
        if let Some(specified_lengths) = specified {
            let intersection = specified_lengths.intersection(*allowed_lengths);
            !intersection.is_empty() // there is at least one common element
        } else {
            true // no specified lengths, so it's allowed
        }
    } else {
        true // no preferred lengths, so it's allowed
    }
}

fn filter_by_vehicle(preferred: &Option<TravelBy>, specified: &Option<TravelBy>) -> bool {
    if let Some(allowed_vehicles) = preferred {
        if let Some(specified_vehicles) = specified {
            allowed_vehicles == specified_vehicles
        } else {
            false // no specified vehicle, so it's not allowed
        }
    } else {
        true // no preferred vehicle, so it's allowed
    }
}

pub trait DestinationExtensions {
    fn rating(&self, month: &Month) -> Rating;
}

impl DestinationExtensions for Destination {
    fn rating(&self, month: &Month) -> Rating {
        self.user_defined_destination
            .month_ratings
            .as_ref()
            .and_then(|ratings| {
                ratings
                    .iter()
                    .find(|(m, _)| m == month)
                    .map(|(_, rating)| *rating)
            })
            .unwrap_or(Rating::NotGood)
    }
}
