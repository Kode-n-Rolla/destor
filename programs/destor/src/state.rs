use crate::constant::{MAX_COLOR_LENGTH, MAX_DESCRIPTION_LENGTH, MAX_MODEL_LENGTH, MAX_NOTE_SIGNERS, MAX_REPORT_HASH_LENGTH, MAX_REPORT_URI_LENGTH};

use super::types::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProtocolConfig {
    pub admin: Pubkey,
    pub bump: u8,
}

impl ProtocolConfig {
    pub const INIT_SPACE: usize =
        8     // discriminator
        + 32  // admin
        + 1;  // bump
}

#[account]
pub struct Vehicle {
    pub vin_hash: [u8; 32],
    pub nft_asset: Pubkey,
    pub manufacturer: Pubkey,
    pub model: String,
    pub manufactured_at: i64, // unix time
    pub owner: Pubkey,
    pub color: String,
    pub mileage: u64,     // is the latest accepted mileage.
    pub next_note_index: u64,  // is used to derive Note PDA addresses.
    pub owner_count: u16, // increments on ownership transfer.
    pub bump: u8,
}

impl Vehicle {
    pub const INIT_SPACE: usize =
        8       // discriminator
        + 32    // vin_hash
        + 32    // nft_asset
        + 32    // manufacturer
        + (4 + MAX_MODEL_LENGTH) // model
        + 8     // manufacturer_at
        + 32    // owner
        + (4 + MAX_COLOR_LENGTH) //color
        + 8     // mileage
        + 8     // next_note_index
        + 2     // onwer_count
        + 1;    // bump
}

#[account]
pub struct Organization {
    pub role: Role,
    pub authority: Pubkey,
    pub pending_authority: Pubkey,
    pub organization_id: [u8; 32],
    pub threshold: u8, // defines how many valid member signatures are required.
    pub active: bool,
    pub bump: u8,
}

impl Organization {
    pub const INIT_SPACE: usize =
        8     // discriminator
        + 1   // role
        + 32  // authority
        + 32  // pending_authority
        + 32  // organization_id
        + 1   // threshold
        + 1   // active
        + 1;  // bump
}

// could be official service, dealer, minter. @todo think about add enum MemberRole
#[account]
pub struct Member {
    pub organization: Pubkey,
    pub wallet: Pubkey,
    pub active: bool,
    pub bump: u8,
}

impl Member {
    pub const INIT_SPACE: usize =
        8     // discriminator
        + 32  // organization
        + 32  // waller
        + 1   // active
        + 1;  // bump
}

#[account]
pub struct Note {
    pub status: Status,
    pub vehicle: Pubkey,
    pub note_index: u64,
    pub role: Role,
    pub note_kind: NoteKind,
    pub mileage: u64,
    pub timestamp: i64,
    pub description: String, // restrict by 200 symbols, e.g.
    pub signers: Vec<Pubkey>,
    pub report_uri: String,  // should have a max length.
    pub report_hash: String, // can verify the off-chain report content.
    pub bump: u8,
}

impl Note {
    pub const INIT_SPACE: usize =
        8     // discriminator
        + 1   // status
        + 32  // vehicle
        + 8   // note_index
        + 1   // role
        + 1   // note_kind
        + 8   // mileage
        + 8   // timestamp
        + (4 + MAX_DESCRIPTION_LENGTH) // description
        + (4 + MAX_NOTE_SIGNERS * 32)  // signers
        + (4 + MAX_REPORT_URI_LENGTH)  // report_uri
        + (4 + MAX_REPORT_HASH_LENGTH) // report_hash
        +1;   // bump 
}
