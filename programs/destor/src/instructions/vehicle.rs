use anchor_lang::prelude::*;
use crate::{
    constant::{MAX_COLOR_LENGTH, MAX_MODEL_LENGTH, MEMBER_SEED, ORGANIZATION_SEED, VEHICLE_SEED}, error::DeStorError, events::{MintedVehicle, TransferredVehicle}, state::{Member, Organization, Vehicle}, types::Role,
};

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct MintVehicle<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
    )]
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
        signer: ctx.accounts.wallet.key(),
        vin_hash: vin_hash,
        vehicle_pda: vehicle.key(),
        timestamp: current_time,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct InitialOwner<'info> {
    pub wallet: Signer<'info>,

    #[account(
        mut,
        seeds = [VEHICLE_SEED, organization.organization_id.as_ref(), vin_hash.as_ref()],
        bump,
    )]
    pub vehicle: Account<'info, Vehicle>,

    #[account(
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
    )]
    pub organization: Account<'info, Organization>,

    #[account(
        seeds = [MEMBER_SEED, organization.key().as_ref(), wallet.key().as_ref()],
        bump,
        has_one = wallet,
    )]
    pub member: Account<'info, Member>,
}

pub fn assign_initial_owner(ctx: Context<InitialOwner>, vin_hash: [u8; 32], new_owner: Pubkey) -> Result<()> {
    require_eq!(ctx.accounts.vehicle.owner, Pubkey::default(), DeStorError::NotOwner);
    require_eq!(ctx.accounts.vehicle.owner_count, 0, DeStorError::InitialOwnerAlreadyAssigned);
    require_eq!(ctx.accounts.organization.key(), ctx.accounts.member.organization.key(), DeStorError::InvalidMember);
    require_eq!(ctx.accounts.organization.role, Role::Manufacturer, DeStorError::InvalidRole);
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);
    require!(ctx.accounts.member.active, DeStorError::MemberIsNotActive);
    require_neq!(new_owner, Pubkey::default(), DeStorError::InvalidPubkey);

    if ctx.accounts.vehicle.vin_hash != vin_hash {
        return err!(DeStorError::InvalidVin);
    }

    let vehicle = &mut ctx.accounts.vehicle;

    vehicle.owner = new_owner;
    vehicle.owner_count = 1;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(TransferredVehicle {
        old_owner: Pubkey::default(),
        new_owner: vehicle.owner,
        vehicle_pda: vehicle.key(),
        timestamp: current_time,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct TransferVehicle<'info> {
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [VEHICLE_SEED, organization.organization_id.as_ref(), vin_hash.as_ref()],
        bump,
    )]
    pub vehicle: Account<'info, Vehicle>,

    #[account(
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
    )]
    pub organization: Account<'info, Organization>,
}

pub fn transfer_vehicle(ctx: Context<TransferVehicle>, vin_hash: [u8; 32], new_owner: Pubkey) -> Result<()> {
    if ctx.accounts.vehicle.vin_hash != vin_hash {
        return err!(DeStorError::InvalidVin);
    }

    require_eq!(ctx.accounts.vehicle.manufacturer.key(), ctx.accounts.organization.key(), DeStorError::InvalidVin);
    require_eq!(ctx.accounts.seller.key(), ctx.accounts.vehicle.owner, DeStorError::NotOwner);
    require_neq!(new_owner, Pubkey::default(), DeStorError::InvalidPubkey);
    require_neq!(new_owner, ctx.accounts.seller.key(), DeStorError::InvalidPubkey);

    let vehicle = &mut ctx.accounts.vehicle;

    let old_owner = vehicle.owner;

    vehicle.owner = new_owner;
    vehicle.owner_count += 1;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(TransferredVehicle {
        old_owner,
        new_owner: vehicle.owner,
        vehicle_pda: vehicle.key(),
        timestamp: current_time,
    });

    Ok(())
}

pub fn verify_owner_transfer() {
    todo!()
}
