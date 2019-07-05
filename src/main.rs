#[cfg(feature = "internal-regenerate")]
use schemafy_core;
#[cfg(feature = "internal-regenerate")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "internal-regenerate")]
schemafy::regenerate!(
    root: Schema
    "src/schema.json"
);

fn main() {}
