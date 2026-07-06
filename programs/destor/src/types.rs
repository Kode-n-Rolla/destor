use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Manufacturer,
    Service,
    RoadInspection,
    Insurance,
    Owner,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum NoteKind {
    Manufacturing,
    Service,
    Inspection,
    Accident,
    InsuranceReport,
    OwnerMaintenance,
    OwnershipTransfer,
}
