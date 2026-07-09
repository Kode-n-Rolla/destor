use anchor_lang::prelude::*;
use crate::{
    constant::{ORGANIZATION_SEED, PROTOCOL_CONFIG_SEED},
    error::DeStorError,
    events::{AcceptedNewAuthority, OrganizationDeactivated, OrganizationRegistered, RequestedNewAuthority, SetOrganizationThreshold},
    state::{Organization, ProtocolConfig},
    types::Role
};

#[derive(Accounts)]
#[instruction(role: Role, organization_id: [u8; 32])]
pub struct RegisterOrganization<'info> {
    #[account(
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        has_one = admin,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(
        init,
        payer = admin,
        space = Organization::INIT_SPACE,
        seeds = [ORGANIZATION_SEED, organization_id.as_ref()],
        bump,
    )]
    pub organization: Account<'info, Organization>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn register_organization(
    ctx: Context<RegisterOrganization>,
    role: Role,
    organization_id: [u8; 32],
    authority: Pubkey,
    threshold: u8
) -> Result<()> {
    require_gt!(threshold, 1, DeStorError::InvalidThresholdValue);

    let organization = &mut ctx.accounts.organization;

    organization.role = role;
    organization.authority = authority.key();
    organization.pending_authority = Pubkey::default();
    organization.organization_id = organization_id;
    organization.threshold = threshold;
    organization.active = true;
    organization.bump = ctx.bumps.organization;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(
        OrganizationRegistered {
            admin: ctx.accounts.admin.key(),
            role: organization.role.clone(),
            organization_pda: ctx.accounts.organization.key(),
            authority: ctx.accounts.organization.authority.key(),
            organization_id: organization_id,
            threshold: threshold,
            timestamp: current_time,
        }
    );
    
    Ok(())
}

#[derive(Accounts)]
pub struct DeactivateOrganization<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump
    )]
    pub organization: Account<'info, Organization>,

    #[account(
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        has_one = admin,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
}

pub fn deactivate_organization(ctx: Context<DeactivateOrganization>) -> Result<()> {
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);

    let accounts = ctx.accounts;

    accounts.organization.active = false;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(
        OrganizationDeactivated {
            admin: accounts.admin.key(),
            organization: accounts.organization.key(),
            timestamp: current_time,
        }
    );

    Ok(())
}

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
        has_one = authority,
    )]
    pub organization: Account<'info, Organization>,
}

pub fn set_organization_threshold(ctx: Context<SetThreshold>, threshold: u8) -> Result<()> {
    require_gt!(threshold, 1, DeStorError::InvalidThresholdValue);
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);

    ctx.accounts.organization.threshold = threshold;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(
        SetOrganizationThreshold {
            authority: ctx.accounts.authority.key(),
            organization: ctx.accounts.organization.key(),
            role: ctx.accounts.organization.role.clone(),
            threshold: threshold,
            timestamp: current_time,
        }
    );

    Ok(())
}

#[derive(Accounts)]
pub struct PendingNewAuthority<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
        has_one = authority,
    )]
    pub organization: Account<'info, Organization>,
}

pub fn request_authority_transfer(ctx: Context<PendingNewAuthority>, new_authority: Pubkey) -> Result<()> {
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);
    require_neq!(new_authority, Pubkey::default(), DeStorError::InvalidPubkey);
    require_neq!(ctx.accounts.organization.authority, new_authority, DeStorError::InvalidPubkey);

    let organization = &mut ctx.accounts.organization;
    organization.pending_authority = new_authority;

    let current_time = Clock::get()?.unix_timestamp;
    
    emit!(
        RequestedNewAuthority {
            current_authority: organization.authority.key(),
            new_authority,
            organization: organization.key(),
            role: organization.role.clone(),
            timestamp: current_time,
        }
    );

    Ok(())
}

#[derive(Accounts)]
pub struct AcceptNewAuthority<'info> {
    #[account(mut)]
    pub pending_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
        has_one = pending_authority,
    )]
    pub organization: Account<'info, Organization>,
}

pub fn accept_authority_transfer(ctx: Context<AcceptNewAuthority>) -> Result<()> {
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);

    let prev_authority = ctx.accounts.organization.authority;

    ctx.accounts.organization.authority = ctx.accounts.pending_authority.key();
    ctx.accounts.organization.pending_authority = Pubkey::default();

    let current_time = Clock::get()?.unix_timestamp;

    emit!(
        AcceptedNewAuthority {
            prev_authority,
            new_authority: ctx.accounts.pending_authority.key(),
            organization: ctx.accounts.organization.key(),
            role: ctx.accounts.organization.role.clone(),
            timestamp: current_time,
        }
    );

    Ok(())
}