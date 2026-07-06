use super::types::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProtocolConfig {
    admin: Pubkey,
    bump: u8,
}

#[account]
pub struct Vehicle {
    vin_hash: [u8; 32],
    nft_asset: Pubkey,
    manufucturer: Pubkey,
    b_day: u64, // unix time
    owner: Pubkey,
    color: String,
    mileage: u64,     // is the latest accepted mileage.
    note_count: u64,  // is used to derive Note PDA addresses.
    owner_count: u16, // increments on ownership transfer.
    bump: u8,
}

impl Vehicle {
    pub const INIT_SPACE: usize = 8 + 32 + 4 + 32 + 8 + 32;
}

#[account]
pub struct Organization {
    role: Role,
    authority: Pubkey,
    threshold: u8, // defines how many valid member signatures are required.
    active: bool,
    bump: u8,
}

#[account]
pub struct Member {
    organization: Pubkey,
    wallet: Pubkey,
    active: bool,
    bump: u8,
}

#[account]
pub struct Note {
    vehicle: Pubkey,
    note_index: u64,
    role: Role,
    note_kind: NoteKind,
    mileaege: u64,
    timestamp: u64,
    description: String, // restrict by 200 symbols, e.g.
    signers: Vec<Pubkey>,
    report_uri: String,  // should have a max length.
    report_hash: String, // can verify the off-chain report content.
    bump: u8,
}
