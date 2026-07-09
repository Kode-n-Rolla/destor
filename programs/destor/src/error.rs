use anchor_lang::prelude::*;

#[error_code]
pub enum DeStorError {
    #[msg("Organization is not active")]
    OrganizationNotActive,
    #[msg("Member is not active")]
    MemberIsNotActive,
    #[msg("Invalid Member")]
    InvalidMember,
    #[msg("Organization and Member did not match")]
    OrgDidntMatchWithMember,
    #[msg("Threshold must be > 1")]
    InvalidThresholdValue,
    #[msg("Invalid Pubkey")]
    InvalidPubkey,
}