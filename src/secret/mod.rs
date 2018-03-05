extern crate base64;
extern crate serde;
extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    #[serde(rename="apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: Metadata,
    #[serde(rename="type")]
    pub resource_type: String,
    pub data: HashMap<String, String>
}

fn horrible_base64_decode(encoded_data: &String) -> String {
    let raw_data = base64::decode(encoded_data).expect("Base64 data was undecodable");
    String::from_utf8(raw_data).expect("Base64 data was not a string")
}

impl Secret {

    /// # Example
    ///
    /// ```
    /// use kubesm::secret::Secret;
    ///
    /// let name = "test-name".to_owned();
    /// let namespace = None;
    /// let source = [
    ///         ("SK_THIS_SHOULD_APPEAR".to_owned(), "win".to_owned()),
    ///         ("THIS_SHOULD_NOT_APPEAR".to_owned(), "fail".to_owned()),
    ///     ].iter().cloned().collect();
    ///
    /// let expected =
    /// "---
    /// apiVersion: v1
    /// kind: Secret
    /// metadata:
    ///   name: \"test-name\"
    ///   namespace: ~
    /// type: Opaque
    /// data:
    ///   THIS_SHOULD_APPEAR: d2lu"
    ///     .to_owned();
    ///
    /// let secret = Secret::new(name, namespace, source);
    ///
    /// assert_eq!(expected, secret.to_yaml());
    /// ```
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

    /// # Example
    ///
    /// ```
    /// use kubesm::secret::Secret;
    ///
    /// let yaml =
    /// "---
    /// apiVersion: v1
    /// kind: Secret
    /// metadata:
    ///   name: \"test-name\"
    ///   namespace: ~
    /// type: Opaque
    /// data:
    ///   THIS_SHOULD_APPEAR: d2lu";
    ///
    /// let secret = Secret::from(yaml).unwrap();
    ///
    /// assert_eq!("v1", secret.api_version.as_str());
    /// assert_eq!("win", secret.data.get("THIS_SHOULD_APPEAR").unwrap().as_str());
    /// ```
    pub fn from(yaml: &str) -> Result<Secret, serde_yaml::Error> {
        let encoded_secret : Secret = serde_yaml::from_str(yaml)?;
        Ok(encoded_secret.decode_data())
    }

    fn get_secrets(source: HashMap<String, String>) -> HashMap<String, String> {
        source.iter()
            // Remove keys that don't start SK_
            .filter(|&(ref key, ref _value)| key.get(..3).unwrap_or_else(|| "") == "SK_".to_string())
            // Remove SK_ from the key
            .map(|(key, value)| (String::from(&key[3..]), value.clone()))
            .collect()
    }

    fn encode_data(&self) -> Secret {
        self.with_different_data(
            self.data.iter()
                .map(|(key, value)| (key.clone(), base64::encode(&value)))
                .collect()
        )
    }

    fn decode_data(&self) -> Secret {
        self.with_different_data(
            self.data.iter()
                .map(|(key, value)| (key.clone(), horrible_base64_decode(&value)))
                .collect()
        )
    }

    fn with_different_data(&self, data: HashMap<String, String>) -> Secret {
        Secret {
            api_version: self.api_version.clone(),
            kind: self.kind.clone(),
            metadata: Metadata {
                name: self.metadata.name.clone(),
                namespace: self.metadata.namespace.clone(),
            },
            resource_type: self.resource_type.clone(),
            data,
        }
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self.encode_data()).expect("Could not serialise secret object")
    }
}
