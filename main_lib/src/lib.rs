use std::sync::Arc;
use shared_lib::*;

pub fn use_common_trait(common_trait: Arc<dyn CommonTrait>) {
    common_trait.foo();
}

uniffi::include_scaffolding!("main_lib");
