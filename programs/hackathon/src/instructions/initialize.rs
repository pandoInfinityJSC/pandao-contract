use crate::schemas::Dao;
use crate::utils::constants::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = DAO_SIZE,
        seeds = [
            DAO_PREFIX.as_ref(),
            authority.key().as_ref()
        ],
        bump,
    )]
    pub dao: Account<'info, Dao>,

    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<Initialize>) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let super_admin = ctx.accounts.authority.key();
    dao.super_admin = super_admin;
    dao.admins = vec![super_admin];

    Ok(())
}
