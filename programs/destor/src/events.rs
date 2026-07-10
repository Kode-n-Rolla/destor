use anchor_lang::prelude::*;

use crate::types::Role;

#[event]
pub struct ProtocolInitialized {
    pub admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OrganizationRegistered {
    pub admin: Pubkey,
    pub role: Role,
    pub organization_pda: Pubkey,
    pub authority: Pubkey,
    pub organization_id: [u8; 32],
    pub threshold: u8,
    pub timestamp: i64,
}

#[event]
pub struct SetOrganizationThreshold {
    pub authority: Pubkey,
    pub organization: Pubkey,
    pub role: Role,
    pub threshold: u8,
    pub timestamp: i64,
}

#[event]
pub struct RequestedNewAuthority {
    pub current_authority: Pubkey,
    pub new_authority: Pubkey,
    pub organization: Pubkey,
    pub role: Role,
    pub timestamp: i64,
}

#[event]
pub struct AcceptedNewAuthority {
    pub prev_authority: Pubkey,
    pub new_authority: Pubkey,
    pub organization: Pubkey,
    pub role: Role,
    pub timestamp: i64,
}

#[event]
pub struct OrganizationDeactivated {
    pub admin: Pubkey,
    pub organization: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OrganizationMemberAdded {
    pub organization_pda: Pubkey,
    pub authority: Pubkey,
    pub member_pda: Pubkey,
    pub member: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OrganizationMemberRemoved {
    pub organization_pda: Pubkey,
    pub authority: Pubkey,
    pub member_pda: Pubkey,
    pub member: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MintedVehicle {
    pub organization_pda: Pubkey,
    pub signer: Pubkey,
    pub vin_hash: [u8; 32],
    pub timestamp: i64,
}
