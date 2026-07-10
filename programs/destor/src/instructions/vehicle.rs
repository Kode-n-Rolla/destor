use anchor_lang::prelude::*;
use crate::{
    constant::{MAX_COLOR_LENGTH, MAX_MODEL_LENGTH, MEMBER_SEED, VEHICLE_SEED}, error::DeStorError, events::MintedVehicle, state::{Member, Organization, Vehicle}, types::Role,
};

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct MintVehicle<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,

    pub organization: Account<'info, Organization>,

    #[account(
        seeds = [MEMBER_SEED, organization.key().as_ref(), wallet.key().as_ref()],
        bump,
        has_one = wallet,
    )]
    pub member: Account<'info, Member>,

    #[account(
        init,
        payer = wallet,
        space = Vehicle::INIT_SPACE,
        seeds = [VEHICLE_SEED, organization.organization_id.as_ref(), vin_hash.as_ref()],
        bump,
    )]
    pub vehicle: Account<'info, Vehicle>,

    pub system_program: Program<'info, System>,
}

pub fn mint_vehicle(
    ctx: Context<MintVehicle>,
    vin_hash: [u8; 32],
    model: String,
    color: String
) -> Result<()> {
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);
    require!(ctx.accounts.member.active, DeStorError::MemberIsNotActive);
    require_eq!(ctx.accounts.organization.role, Role::Manufacturer, DeStorError::InvalidRole);
    require_eq!(ctx.accounts.organization.key(), ctx.accounts.member.organization);
    require_gte!(MAX_MODEL_LENGTH, model.len(), DeStorError::StringToLong);
    require_gte!(MAX_COLOR_LENGTH, color.len(), DeStorError::StringToLong);

    let vehicle = &mut ctx.accounts.vehicle;
    let current_time = Clock::get()?.unix_timestamp;

    vehicle.vin_hash = vin_hash;
    vehicle.nft_asset = Pubkey::default(); // for now. @todo upgrade
    vehicle.manufacturer = ctx.accounts.organization.key();
    vehicle.model = model;
    vehicle.manufactured_at = current_time;
    vehicle.owner = Pubkey::default();
    vehicle.color = color;
    vehicle.mileage = 0;
    vehicle.note_count = 0;
    vehicle.owner_count = 0;
    vehicle.bump = ctx.bumps.vehicle;

    emit!(MintedVehicle {
        organization_pda: ctx.accounts.organization.key(),
        signer: ctx.accounts.member.key(),
        vin_hash: vin_hash,
        vehicle_pda: vehicle.key(),
        timestamp: current_time,
    });

    Ok(())
}
