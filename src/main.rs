#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate serde;
extern crate serde_yaml;

use std::collections::HashMap;
use std::env::{vars, var};

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Secret {
    #[serde(rename="apiVersion")]
    api_version: String,
    kind: String,
    metadata: Metadata,
    #[serde(rename="type")]
    resource_type: String,
    data: HashMap<String, String>
}


fn main() {
    let mut secret = Secret {
        api_version: "v1".to_string(),
        kind: "Secret".to_string(),
        metadata: Metadata {
            name: var("NAME").expect("You must pass NAME in the environment"),
        },
        resource_type: "Opaque".to_string(),
        data: HashMap::new(),
    };

    vars().filter(|&(ref key, ref _value)| key.get(0..3).unwrap_or_else(|| "") == "SK_".to_string())
        .map(|(key, value)| (String::from(&key[3..]), base64::encode(&value)))
        .for_each(|(key, value)| { secret.data.insert(key.clone(), value.clone()); });


    let serialized = serde_yaml::to_string(&secret).unwrap();

    println!("{}", serialized);
}

