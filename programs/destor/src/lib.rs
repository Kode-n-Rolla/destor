use anchor_lang::prelude::*;

use crate::types::Role;
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
        instructions::initialize_protocol::initialize_protocol(ctx)
    }

    pub fn register_organization(
        ctx: Context<RegisterOrganization>,
        role: Role,
        threshold: u8
    ) -> Result<()> {
        instructions::register_organization::register_organization(ctx, role, threshold)
    }

    pub fn add_organization_member(ctx: Context<AddMember>, wallet: Pubkey) -> Result<()> {
        instructions::add_organization_member::add_organization_member(ctx, wallet)
    }
}

