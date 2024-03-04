use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

// Addresses are 32-bytes long
type Byte = u8;
const LEN: usize = 32;

#[derive(Error, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AddressError {
    #[error("Invalid format")]
    InvalidFormat,

    #[error("Invalid length")]
    InvalidLength,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into="String")]
pub struct Address([Byte; LEN]);

impl TryFrom<Vec<Byte>> for Address {
    type Error = AddressError;

    fn try_from(vec: Vec<Byte>) -> Result<Self, AddressError> {
        let slice = vec.as_slice();
        match slice.try_into() {
            Ok(byte_array) => Ok(Address(byte_array)),
            Err(_) => Err(AddressError::InvalidLength),
        }
    }
}

impl