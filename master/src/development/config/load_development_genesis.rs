use super::{
    decode_development_spec::decode_development_spec, read_bounded_input::read_bounded_input,
};
use anyhow::Result;
use eve_protocol_config::genesis::DevelopmentGenesis;
use std::path::Path;

pub fn load_development_genesis(path: &Path) -> Result<DevelopmentGenesis> {
    decode_development_spec(&read_bounded_input(path, 1_048_576)?)
}
