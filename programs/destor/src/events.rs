use anchor_lang::prelude::*;

use crate::types::{NoteKind, Role};

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
pub struct OrganizationMemberReactivate {
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
    pub vehicle_pda: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TransferredVehicle {
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
    pub vehicle_pda: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AddedNote {
    pub signer: Pubkey,
    pub vehicle_pda: Pubkey,
    pub note_pda: Pubkey,
    pub note_index: u64,
    pub role: Role,
    pub note_kind: NoteKind,
    pub timestamp: i64,
}
