use destiny_model::destiny::common::types::StoreName;
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
    #[route("/")]
    StoreList,
    #[route("/store/edit/:name")]
    StoreEdit { name: StoreName },
    #[route("/store/:name")]
    StoreView { name: StoreName },
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
            .json::<Vec<StoreName>>()
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
                    class: "new-store",
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
                { store_names.cloned().unwrap_or_default().iter().map(|store_name| {
                    store_list_element(store_name.clone())
                })}
            }
        }
    }
}

pub fn store_list_element(store_name: StoreName) -> Element {
    rsx! {
        li {
            span { {store_name.clone()} }
            span {
                Link {
                    to: Route::StoreEdit { name: {store_name.clone()} },
                    "Edit"
                }
            }
            span {
                Link {
                    to: Route::StoreView { name: {store_name.clone()} },
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
pub fn StoreEdit(name: StoreName) -> Element {
    rsx! {
        div {
            id: "storeedit",
            header { class: "header",
                h1 { {name} }
            }
        }
    }
}

#[component]
pub fn StoreView(name: StoreName) -> Element {
    rsx! {
        div {
            id: "storeview",
            header { class: "header",
                h1 { {name} }
            }
        }
    }
}
