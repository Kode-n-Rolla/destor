use anchor_lang::prelude::*;

use crate::constant::PROTOCOL_CONFIG_SEED;
use crate::state::ProtocolConfig;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(
        init,
        payer = admin,
        space = ProtocolConfig::INIT_SPACE,
        seeds = [PROTOCOL_CONFIG_SEED],
        bump,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProtocol>) -> Result<()> {
    let protocol_config = &mut ctx.accounts.protocol_config;

    protocol_config.admin = ctx.accounts.admin.key();
    protocol_config.bump = ctx.bumps.protocol_config;

    Ok(())
}