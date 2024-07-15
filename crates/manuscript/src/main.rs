use crate::default_trait::run_default_trait;
use crate::must_use::run_must_use;

pub mod default_trait;
pub mod must_use;

fn main() {
    println!("Hello, world!");

    run_default_trait();
    run_must_use();
}
