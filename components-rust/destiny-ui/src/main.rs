use destiny_model::destiny::common::types::{StoreName, User};
use dioxus::prelude::*;
use reqwest::Client;

const DESTINY_BASE_URL: &str = "http://localhost:9006";

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/ui")]
    StoreList,
    #[route("/ui/store/edit/:owner/:name")]
    StoreEdit { owner: User, name: StoreName },
    #[route("/ui/store/:owner/:name")]
    StoreView { owner: User, name: StoreName },
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn StoreList() -> Element {
    let mut store_names = use_resource(|| async move {
        Client::builder()
            .build()
            .expect("Could not build client")
            .get(format!("{DESTINY_BASE_URL}/api/stores"))
            .header("Accept", "application/json")
            .send()
            .await
            .expect("Failed to fetch stores")
            .json::<Vec<(User, StoreName)>>()
            .await
            .expect("Failed to decode list of stores")
    });

    let mut new_store_name = use_signal(|| "".to_string());

    rsx! {
        div {
            id: "storelist",
            header { class: "header",
                h1 { "Stores" }

                input {
                    class: "text-edit",
                    placeholder: "New store name",
                    value: "",
                    autofocus: "true",
                    oninput: move |evt| new_store_name.set(evt.value()),
                    onkeydown: move |evt| async move {
                        if evt.key() == Key::Enter && !new_store_name.read().is_empty() {
                            let _ = create_new_store(&new_store_name.read()).await?;
                            store_names.restart();
                            new_store_name.set("".to_string());
                        }
                        Ok(())
                    }
                }
            }
            ul { id: "stores",
                { store_names.cloned().unwrap_or_default().iter().map(|(owner, store_name)| {
                    store_list_element(owner.clone(), store_name.clone())
                })}
            }
        }
    }
}

pub fn store_list_element(owner: User, store_name: StoreName) -> Element {
    rsx! {
        li {
            span { {store_name.clone()} }
            span {
                Link {
                    to: Route::StoreEdit { owner: {owner.clone()}, name: {store_name.clone()} },
                    "Edit"
                }
            }
            span {
                Link {
                    to: Route::StoreView { owner: {owner.clone()}, name: {store_name.clone()} },
                    "Open"
                }
            }
        }
    }
}

async fn create_new_store(name: &str) -> Result<(), reqwest::Error> {
    Client::builder()
        .build()
        .expect("Could not build client")
        .post(format!("{DESTINY_BASE_URL}/api/stores"))
        .json(name)
        .header("Accept", "application/json")
        .send()
        .await?;
    Ok(())
}

#[component]
pub fn StoreEdit(owner: User, name: StoreName) -> Element {
    let name_clone = name.clone();
    let owner_clone = owner.clone();
    let mut currency = use_resource(move || {
        let name_clone = name_clone.clone();
        let owner_clone = owner_clone.clone();
        async move {
            Client::builder()
                .build()
                .expect("Could not build client")
                .get(format!(
                    "{DESTINY_BASE_URL}/api/stores/{owner_clone}/{name_clone}/currency"
                ))
                .header("Accept", "application/json")
                .send()
                .await
                .expect("Failed to fetch stores")
                .json::<String>()
                .await
                .expect("Failed to decode list of stores")
        }
    });

    let mut new_currency = use_signal(|| "".to_string());

    let name_clone = name.clone();
    let owner_clone = owner.clone();

    rsx! {
        div {
            id: "storeedit",
            header { class: "header",
                h1 { {name} }
            }
            div {
                id: "property",
                label { r#for: "currency", "Currency for estimates" }
                input {
                    id: "currency",
                    class: "text-edit",
                    r#type: "text",
                    value: currency.cloned().unwrap_or_default(),
                    disabled: currency.cloned().is_none(),
                    oninput: move |evt| new_currency.set(evt.value()),
                    onkeydown: move |evt| {
                        let name_clone = name_clone.clone();
                        let owner_clone = owner_clone.clone();
                        async move {
                            if evt.key() == Key::Enter && !new_currency.read().is_empty() {
                                let _ = set_currency(owner_clone, name_clone, &new_currency.read()).await?;
                                currency.restart();
                                new_currency.set("".to_string());
                            }
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}

async fn set_currency(owner: User, name: StoreName, currency: &str) -> Result<(), reqwest::Error> {
    Client::builder()
        .build()
        .expect("Could not build client")
        .put(format!(
            "{DESTINY_BASE_URL}/api/stores/{owner}/{name}/currency"
        ))
        .json(currency)
        .header("Accept", "application/json")
        .send()
        .await?;
    Ok(())
}

#[component]
pub fn StoreView(owner: User, name: StoreName) -> Element {
    rsx! {
        div {
            id: "storeview",
            header { class: "header",
                h1 { {name} }
            }
        }
    }
}
