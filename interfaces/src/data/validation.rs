use intmax2_zkp::common::signature::key_set::KeySet;

use super::error::DataError;

pub trait Validation {
    fn validate(&self, key: KeySet) -> Result<(), DataError>;
}
