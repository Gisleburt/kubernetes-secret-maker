extern crate base64;
extern crate serde;
extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    #[serde(rename="apiVersion")]
    api_version: String,
    kind: String,
    metadata: Metadata,
    #[serde(rename="type")]
    resource_type: String,
    data: HashMap<String, String>
}

impl Secret {
    pub fn new(name: String, namespace: Option<String>, source: HashMap<String, String>) -> Secret {
        Secret {
            api_version: "v1".to_string(),
            kind: "Secret".to_string(),
            metadata: Metadata {
                name,
                namespace,
            },
            resource_type: "Opaque".to_string(),
            data: Secret::get_secrets(source),
        }
    }

    fn get_secrets(source: HashMap<String, String>) -> HashMap<String, String> {
        source.iter()
            // Remove keys that don't start SK_
            .filter(|&(ref key, ref _value)| key.get(..3).unwrap_or_else(|| "") == "SK_".to_string())
            // Remove SK_ from the key and base64 encode the value
            .map(|(key, value)| (String::from(&key[3..]), base64::encode(&value)))
            .collect()
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).expect("Could not serialise secret object")
    }
}

#[cfg(test)]
mod tests {
    use super::Secret;

    #[test]
    fn e2e() {
        let name = "test-name".to_owned();
        let namespace = None;
        let source = [
            ("SK_THIS_SHOULD_APPEAR".to_owned(), "win".to_owned()),
            ("THIS_SHOULD_NOT_APPEAR".to_owned(), "fail".to_owned()),
        ].iter().cloned().collect();

        let expected =
"---
apiVersion: v1
kind: Secret
metadata:
  name: \"test-name\"
  namespace: ~
type: Opaque
data:
  THIS_SHOULD_APPEAR: d2lu".to_owned();

        assert_eq!(expected, Secret::new(name, namespace, source).to_yaml());
    }
}
