use anyhow::anyhow;
use golem_client::api::{ComponentClient, ComponentClientLive, WorkerClient, WorkerClientLive};
use golem_client::model::InvokeParameters;
use golem_client::Context;
use golem_wasm_rpc::{IntoValueAndType, Value};
use reqwest::Url;
use test_r::test;
use uuid::Uuid;

test_r::enable!();

#[test]
async fn create_store_via_user() -> anyhow::Result<()> {
    // Assuming the server is running and the components are deployed

    let client = new_reqwest_client()?;

    let component_client = ComponentClientLive {
        context: Context {
            client: client.clone(),
            base_url: Url::parse("http://localhost:9881")?,
        },
    };
    let worker_client = WorkerClientLive {
        context: Context {
            client: client.clone(),
            base_url: Url::parse("http://localhost:9881")?,
        },
    };

    let user_component_id = component_client
        .get_components(Some("destiny:user"))
        .await
        .map_err(|err| anyhow!(format!("{err:?}")))?
        .first()
        .unwrap()
        .versioned_component_id
        .component_id;
    let store_component_id = component_client
        .get_components(Some("destiny:store"))
        .await
        .map_err(|err| anyhow!(format!("{err:?}")))?
        .first()
        .unwrap()
        .versioned_component_id
        .component_id;

    let user1 = format!("{}@test.com", Uuid::new_v4());

    let initial_store_list: Value = worker_client
        .invoke_and_await_function(
            &user_component_id,
            &user1,
            None,
            "destiny::user-exports/destiny-user-api.{stores}",
            &InvokeParameters { params: vec![] },
        )
        .await
        .map_err(|err| anyhow!(format!("{err:?}")))?
        .result
        .try_into()
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    let create_store_result: Value = worker_client
        .invoke_and_await_function(
            &user_component_id,
            &user1,
            None,
            "destiny::user-exports/destiny-user-api.{create-store}",
            &InvokeParameters {
                params: vec!["store1".into_value_and_type().try_into().unwrap()],
            },
        )
        .await
        .map_err(|err| anyhow!(format!("{err:?}")))?
        .result
        .try_into()
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    let updated_store_list: Value = worker_client
        .invoke_and_await_function(
            &user_component_id,
            &user1,
            None,
            "destiny::user-exports/destiny-user-api.{stores}",
            &InvokeParameters { params: vec![] },
        )
        .await
        .map_err(|err| anyhow!(format!("{err:?}")))?
        .result
        .try_into()
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    let initial_home_location: Value = worker_client
        .invoke_and_await_function(
            &store_component_id,
            &format!("{user1}/store1"),
            None,
            &format!(
                "destiny::store-exports/destiny-store-api.{{store(\"{user1}\").get-home-location}}"
            ),
            &InvokeParameters { params: vec![] },
        )
        .await
        .map_err(|err| anyhow!(format!("{err:?}")))?
        .result
        .try_into()
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    assert_eq!(initial_store_list, Value::List(Vec::new()));
    assert_eq!(create_store_result, Value::Bool(true));
    assert_eq!(updated_store_list, Value::List(vec![Value::String("store1".to_string())]));
    assert_eq!(initial_home_location, Value::String("Kosd, Hungary".to_string()));

    Ok(())
}

fn new_reqwest_client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?)
}

//     ║ Exports:
//     ║   destiny:store-exports/destiny-store-api.{[constructor]store}(user: string) -> handle<0>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.set-currency}(self: &handle<0>, currency: string) -> result<_, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.get-currency}(self: &handle<0>) -> result<string, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.set-home-location}(self: &handle<0>, location: string) -> result<_, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.get-home-location}(self: &handle<0>) -> result<string, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.add-destination}(self: &handle<0>, name: string, destination: record { approximated-travel-cost: option<u32>, approximated-daily-cost: option<u32>, lengths: option<flags { weekend, long-weekend, week, two-weeks, three-weeks }>, month-ratings: option<list<tuple<enum { january, february, march, april, may, june, july, august, september, october, november, december }, enum { not-good, good, best }>>>, description: option<string>, travel-by: option<flags { car, motorbike, plane, train }> }) -> result<_, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.update-destination}(self: &handle<0>, name: string, destination: record { approximated-travel-cost: option<u32>, approximated-daily-cost: option<u32>, lengths: option<flags { weekend, long-weekend, week, two-weeks, three-weeks }>, month-ratings: option<list<tuple<enum { january, february, march, april, may, june, july, august, september, october, november, december }, enum { not-good, good, best }>>>, description: option<string>, travel-by: option<flags { car, motorbike, plane, train }> }) -> result<_, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.get-destination}(self: &handle<0>, name: string) -> result<option<record { name: string, user-defined-destination: record { approximated-travel-cost: option<u32>, approximated-daily-cost: option<u32>, lengths: option<flags { weekend, long-weekend, week, two-weeks, three-weeks }>, month-ratings: option<list<tuple<enum { january, february, march, april, may, june, july, august, september, october, november, december }, enum { not-good, good, best }>>>, description: option<string>, travel-by: option<flags { car, motorbike, plane, train }> } }>, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.get-destinations}(self: &handle<0>) -> result<list<record { name: string, user-defined-destination: record { approximated-travel-cost: option<u32>, approximated-daily-cost: option<u32>, lengths: option<flags { weekend, long-weekend, week, two-weeks, three-weeks }>, month-ratings: option<list<tuple<enum { january, february, march, april, may, june, july, august, september, october, november, december }, enum { not-good, good, best }>>>, description: option<string>, travel-by: option<flags { car, motorbike, plane, train }> } }>, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.remove-destination}(self: &handle<0>, name: string) -> result<_, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{[method]store.get-ordered-destinations}(self: &handle<0>, preferences: record { month: enum { january, february, march, april, may, june, july, august, september, october, november, december }, lengths: option<flags { weekend, long-weekend, week, two-weeks, three-weeks }>, travel-by: option<flags { car, motorbike, plane, train }> }) -> result<list<record { name: string, user-defined-destination: record { approximated-travel-cost: option<u32>, approximated-daily-cost: option<u32>, lengths: option<flags { weekend, long-weekend, week, two-weeks, three-weeks }>, month-ratings: option<list<tuple<enum { january, february, march, april, may, june, july, august, september, october, november, december }, enum { not-good, good, best }>>>, description: option<string>, travel-by: option<flags { car, motorbike, plane, train }> } }>, variant { not-found(string), already-exists(string), access-denied, not-initialized }>
//     ║   destiny:store-exports/destiny-store-api.{initialize}(user: string) -> bool
//     ║   destiny:store-exports/destiny-store-api.{[drop]store}(self: handle<0>)

//   ║ Exports:
//   ║   destiny:user-exports/destiny-user-api.{create-store}(name: string) -> result<_, variant { already-exists }>
//   ║   destiny:user-exports/destiny-user-api.{stores}() -> list<string>
