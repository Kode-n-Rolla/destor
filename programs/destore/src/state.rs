use anchor_lang::prelude::*;

#[account]
pub struct Vehicle {
    vin: String,
    manufucture_by: Pubkey,
    color: String,
    d_day: u64, // unix time 
    owner: Pubkey,
    mileage: u64,
}

impl Vehicle {
    pub const INIT_SPACE: usize = 8 + 32 + 4 +32 + 8 + 32;
}

#[account]
pub struct Note {

}