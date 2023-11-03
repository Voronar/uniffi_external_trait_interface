pub trait CommonTrait: Send + Sync {
    fn foo(&self);
}

uniffi::include_scaffolding!("shared_lib");
