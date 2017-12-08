#[macro_use]
extern crate serde_derive;

mod secret;
use secret::Secret;

use std::env::{vars, var};

fn main() {
    let secret_name = var("NAME").expect("You must pass NAME in the environment");
    let secret = Secret::new(secret_name, vars().collect());
    println!("{}", secret.to_yaml());
}
