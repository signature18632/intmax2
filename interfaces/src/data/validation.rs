use intmax2_zkp::common::signature::key_set::KeySet;

pub trait Validation {
    fn validate(&self, key: KeySet) -> anyhow::Result<()>;
}
