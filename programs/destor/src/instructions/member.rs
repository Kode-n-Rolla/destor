use anchor_lang::prelude::*;

use crate::{
    constant::MEMBER_SEED, error::DeStorError, events::{OrganizationMemberAdded, OrganizationMemberRemoved}, state::{Member, Organization}
};

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Member::INIT_SPACE,
        seeds = [MEMBER_SEED, organization.key().as_ref(), wallet.as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    #[account(
        has_one = authority,
    )]
    pub organization: Account<'info, Organization>,

    pub system_program: Program<'info, System>,
}

pub fn add_organization_member(ctx: Context<AddMember>, wallet: Pubkey) -> Result<()> {
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);

    let member = &mut ctx.accounts.member;
    member.organization = ctx.accounts.organization.key();
    member.wallet = wallet;
    member.active = true;
    member.bump = ctx.bumps.member;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(
        OrganizationMemberAdded {
            organization_pda: ctx.accounts.organization.key(),
            authority: ctx.accounts.authority.key(),
            member_pda: ctx.accounts.member.key(),
            member: wallet,
            timestamp: current_time,
        }
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct RemoveMember<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MEMBER_SEED, organization.key().as_ref(), wallet.as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    #[account(
        has_one = authority,
    )]
    pub organization: Account<'info, Organization>,
}

pub fn remove_organization_member(ctx: Context<RemoveMember>, wallet: Pubkey) -> Result<()> {
    let accounts = ctx.accounts;
    require!(accounts.organization.active, DeStorError::OrganizationNotActive);
    require!(accounts.member.active, DeStorError::MemberIsNotActive);

    require_eq!(accounts.member.organization, accounts.organization.key(), DeStorError::InvalidMember);
    require_eq!(accounts.member.wallet, wallet, DeStorError::InvalidMember);

    accounts.member.active = false;

    let current_time = Clock::get()?.unix_timestamp;

    emit!(
        OrganizationMemberRemoved {
            organization_pda: accounts.organization.key(),
            authority: accounts.authority.key(),
            member_pda: accounts.member.key(),
            member: wallet,
            timestamp: current_time,
        }
    );

    Ok(())
}