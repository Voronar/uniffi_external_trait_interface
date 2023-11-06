#[uniffi::export]
pub trait CommonTrait: Send + Sync {
    fn foo(&self) -> String;
}

uniffi::setup_scaffolding!();
