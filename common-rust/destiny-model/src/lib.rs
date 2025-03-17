use wit_bindgen::generate;

generate!({
    path: "../../wit",
    // additional_derives: [serde::Deserialize, serde::Serialize],
    generate_all,
    generate_unused_types: true
});

