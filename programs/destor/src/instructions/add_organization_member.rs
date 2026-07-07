use anchor_lang::prelude::*;
use crate::{
    constant::MEMBER_SEED,
    error::DeStoreError,
    state::{Member, Organization},
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
    require!(ctx.accounts.organization.active, DeStoreError::OrganizationNotActive);

    let member = &mut ctx.accounts.member;
    member.organization = ctx.accounts.organization.key();
    member.wallet = wallet;
    member.active = true;
    member.bump = ctx.bumps.member;

    // @todo add event

    Ok(())
}