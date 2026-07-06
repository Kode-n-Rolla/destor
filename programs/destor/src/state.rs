use super::types::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProtocolConfig {
    pub admin: Pubkey,
    pub bump: u8,
}

impl ProtocolConfig {
    pub const INIT_SPACE: usize = 8 + 32 + 1;
}

#[account]
pub struct Vehicle {
    pub vin_hash: [u8; 32],
    pub nft_asset: Pubkey,
    pub manufacturer: Pubkey,
    pub manufactured_at: u64, // unix time
    pub owner: Pubkey,
    pub color: String,
    pub mileage: u64,     // is the latest accepted mileage.
    pub note_count: u64,  // is used to derive Note PDA addresses.
    pub owner_count: u16, // increments on ownership transfer.
    pub bump: u8,
}

impl Vehicle {
    pub const INIT_SPACE: usize = 8 + 32 + 4 + 32 + 8 + 32;
}

#[account]
pub struct Organization {
    pub role: Role,
    pub authority: Pubkey,
    pub threshold: u8, // defines how many valid member signatures are required.
    pub active: bool,
    pub bump: u8,
}

impl Organization {
    pub const INIT_SPACE: usize = 8 + 1 + 32 + 1 + 1 + 1;
}

#[account]
pub struct Member {
    pub organization: Pubkey,
    pub wallet: Pubkey,
    pub active: bool,
    pub bump: u8,
}

#[account]
pub struct Note {
    pub vehicle: Pubkey,
    pub note_index: u64,
    pub role: Role,
    pub note_kind: NoteKind,
    pub mileage: u64,
    pub timestamp: u64,
    pub description: String, // restrict by 200 symbols, e.g.
    pub signers: Vec<Pubkey>,
    pub report_uri: String,  // should have a max length.
    pub report_hash: String, // can verify the off-chain report content.
    pub bump: u8,
}
