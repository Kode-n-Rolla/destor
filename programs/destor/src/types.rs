use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Manufacturer,
    Service,
    RoadInspection,
    Insurance,
    Owner,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Role::Insurance => "Insurance",
            Role::Manufacturer => "Manufacturer",
            Role::Owner => "Owner",
            Role::RoadInspection => "Road Inspection",
            Role::Service => "Serivce Station",
        };

        write!(f, "{}", role_str)
    }
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    Approved,
    Rejected,
}
