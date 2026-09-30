//! Machine-generated bindings from the exact pinned upstream protobuf sources.
//!
//! Output is generated into Cargo's untracked `OUT_DIR`; handwritten domain
//! behavior belongs outside these binding modules.

pub mod abci {
    include!(concat!(env!("OUT_DIR"), "/tendermint.abci.rs"));
}
pub mod crypto {
    include!(concat!(env!("OUT_DIR"), "/tendermint.crypto.rs"));
}
pub mod types {
    include!(concat!(env!("OUT_DIR"), "/tendermint.types.rs"));
}
pub mod version {
    include!(concat!(env!("OUT_DIR"), "/tendermint.version.rs"));
}
