use anchor_lang::prelude::*;

use crate::types::Role;
use instructions::*;

pub mod constant;
pub mod events;
pub mod error;
pub mod instructions;
pub mod state;
pub mod types;

declare_id!("FWrEp5K7BfRzJ5wK8w6yoKGKWKWHE96dSEuZheRwDCyH");

#[program]
pub mod destor {
    use super::*;

    pub fn initialize(ctx: Context<InitializeProtocol>) -> Result<()> {
        instructions::protocol::initialize_protocol(ctx)
    }

    // organization_id - hash from name of the organization. Compute at the frontend
    pub fn register_organization(
        ctx: Context<RegisterOrganization>,
        role: Role,
        organization_id: [u8; 32],
        authority: Pubkey,
        threshold: u8
    ) -> Result<()> {
        instructions::organization::register_organization(ctx, role, organization_id, authority, threshold)
    }

    pub fn set_organization_threshold(ctx: Context<SetThreshold>, threshold: u8) -> Result<()> {
        instructions::organization::set_organization_threshold(ctx, threshold)
    }

    pub fn request_authority_transfer(ctx: Context<PendingNewAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::organization::request_authority_transfer(ctx, new_authority)
    }

    pub fn accept_authority_transfer(ctx: Context<AcceptNewAuthority>) -> Result<()> {
        instructions::organization::accept_authority_transfer(ctx)
    }

    pub fn deactivate_organization(ctx: Context<DeactivateOrganization>) -> Result<()> {
        instructions::organization::deactivate_organization(ctx)
    }

    pub fn add_organization_member(ctx: Context<AddMember>, wallet: Pubkey) -> Result<()> {
        instructions::member::add_organization_member(ctx, wallet)
    }

    pub fn remove_organization_member(ctx: Context<RemoveMember>, wallet: Pubkey) -> Result<()> {
        instructions::member::remove_organization_member(ctx, wallet)
    }

    pub fn mint_vehicle(
        ctx: Context<MintVehicle>,
        vin_hash: [u8; 32],
        model: String,
        color: String
    ) -> Result<()> {
        instructions::vehicle::mint_vehicle(ctx, vin_hash, model, color)
}
}

