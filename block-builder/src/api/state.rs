use crate::EnvVar;

use super::block_builder::BlockBuilder;

#[derive(Debug, Clone)]
pub struct State {
    pub block_builder: BlockBuilder,
}

impl State {
    pub fn new(env: &EnvVar) -> Self {
        let block_builder = BlockBuilder::new(env);
        State { block_builder }
    }

    pub fn run(&self) {
        self.block_builder.run();
    }
}
