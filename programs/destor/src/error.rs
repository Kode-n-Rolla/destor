use anchor_lang::prelude::*;

#[error_code]
pub enum DeStorError {
    #[msg("Organization is not active")]
    OrganizationNotActive,
    #[msg("Member is not active")]
    MemberIsNotActive,
}