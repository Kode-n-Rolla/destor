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
    pub threshold: u8,
}

#[event]
pub struct OrganizationMemberAdded {
    pub organization_pda: Pubkey,
    pub authority: Pubkey,
    pub member_pda: Pubkey,
    pub member: Pubkey,
}