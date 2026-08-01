use anchor_lang::prelude::*;

#[error_code]
pub enum DeStorError {
    #[msg("Organization is not active")]
    OrganizationNotActive,
    #[msg("Member is not active")]
    MemberIsNotActive,
    #[msg("Member is active")]
    MemberIsActive,
    #[msg("Invalid Member")]
    InvalidMember,
    #[msg("Organization and Member did not match")]
    OrgDidntMatchWithMember,
    #[msg("Threshold must be > 1")]
    InvalidThresholdValue,
    #[msg("Invalid Pubkey")]
    InvalidPubkey,
    #[msg("Invalid Role")]
    InvalidRole,
    #[msg("Description is too long")]
    DescriptionToLong,
    #[msg("Report URI is too long")]
    ReportUriToLong,
    #[msg("Report hash is too long")]
    ReportHashToLong,
    #[msg("Model hash is too long")]
    ModelToLong,
    #[msg("Color hash is too long")]
    ColorToLong,
    #[msg("Invalid VIN")]
    InvalidVin,
    #[msg("Not Owner")]
    NotOwner,
    #[msg("Initial Owner Already Assigned")]
    InitialOwnerAlreadyAssigned,
    #[msg("Vehicle and note does not match")]
    InvalidVehicleOrNote,
    #[msg("Invalid Note")]
    InvalidNote,
    #[msg("Member already signed the note")]
    MemberAlreadySigner,
    #[msg("Mileage must be greater than the previous mileage")]
    InvalidMileage,
}
