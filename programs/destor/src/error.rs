use anchor_lang::prelude::*;

#[error_code]
pub enum DeStoreError {
    #[msg("Organization is not active")]
    OrganizationNotActive,
    //#[msg("")]
}