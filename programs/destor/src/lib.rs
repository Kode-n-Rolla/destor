use anchor_lang::prelude::*;
use instructions::*;

pub mod constant;
pub mod error;
pub mod instructions;
pub mod state;
pub mod types;

declare_id!("FWrEp5K7BfRzJ5wK8w6yoKGKWKWHE96dSEuZheRwDCyH");

#[program]
pub mod destor {
    use super::*;

    pub fn initialize(ctx: Context<InitializeProtocol>) -> Result<()> {
        instructions::initialize_protocol::handler(ctx);
        
        Ok(())
    }
}

