use anchor_lang::prelude::*;
use crate::{
    constant::{ORGANIZATION_SEED, PROTOCOL_CONFIG_SEED},
    events::OrganizationRegistered,
    state::{Organization, ProtocolConfig},
    types::Role,
};

#[derive(Accounts)]
#[instruction(role: Role)]
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
        seeds = [ORGANIZATION_SEED, authority.key().as_ref()],
        bump,
    )]
    pub organization: Account<'info, Organization>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: authority is just stored as organization authority for now
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn register_organization(ctx: Context<RegisterOrganization>, role: Role, threshold: u8) -> Result<()> {
    let organization = &mut ctx.accounts.organization;

    organization.role = role;
    organization.authority = ctx.accounts.authority.key();
    organization.threshold = threshold;
    organization.active = true;
    organization.bump = ctx.bumps.organization;

    emit!(
        OrganizationRegistered {
            admin: ctx.accounts.admin.key(),
            role: organization.role.clone(),
            organization_pda: ctx.accounts.organization.key(),
            authority: ctx.accounts.organization.authority.key(),
            threshold: threshold,
        }
    );
    
    Ok(())
}

pub fn deactivate_organization() {
    todo!();
}

pub fn set_organization_threshold() {
    todo!();
}

pub fn change_organization_authority() {
    todo!();
}