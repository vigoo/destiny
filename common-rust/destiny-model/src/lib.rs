use wit_bindgen::generate;

generate!({
    path: "../../wit",
    // additional_derives: [serde::Deserialize, serde::Serialize],
    generate_all,
    generate_unused_types: true
});

use crate::destiny::common::types::*;

pub fn store_worker_name(owner: &User, store_name: &StoreName) -> String {
    format!("{}/{}", owner, store_name)
}