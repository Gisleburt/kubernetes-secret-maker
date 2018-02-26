extern crate kubesm;

use kubesm::secret::Secret;

use std::env::{vars, var};

fn main() {
    let secret_name = var("NAME").expect("You must pass NAME in the environment");
    let secret_namespace = var("NAMESPACE").ok();
    let secret = Secret::new(secret_name, secret_namespace, vars().collect());
    println!("{}", secret.to_yaml());
}
