use shared_lib::*;
use std::sync::Arc;

// External trait doesn't work
#[uniffi::export]
pub fn use_common_trait(common_trait: Arc<dyn CommonTrait>) {
    println!("{}", common_trait.foo());
}

// Local trait works
// #[uniffi::export]
// pub trait CommonTrait1: Send + Sync {
//     fn foo1(&self) -> String;
// }

// #[uniffi::export]
// pub fn use_common_trait1(common_trait: Arc<dyn CommonTrait1>) {
//     println!("{}", common_trait.foo1());
// }

uniffi::setup_scaffolding!();
