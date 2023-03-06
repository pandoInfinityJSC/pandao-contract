use crate::schemas::Dao;
use crate::utils::constants::*;
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetAdmins<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            DAO_PREFIX.as_ref(),
            authority.key().as_ref()
        ],
        bump,
        constraint = authority.key() == dao.super_admin.key()
    )]
    pub dao: Account<'info, Dao>,

    /// CHECK: Just a pure account
    pub account: AccountInfo<'info>,
}

pub fn grant_admin(ctx: Context<SetAdmins>) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    if dao.has_admin(ctx.accounts.account.key()) {
        return Err(ErrorCode::AccountIsAnAdmin.into());
    }

    dao.admins.push(ctx.accounts.account.key());

    Ok(())
}

pub fn revoke_admin(ctx: Context<SetAdmins>) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    if !dao.has_admin(ctx.accounts.account.key()) {
        return Err(ErrorCode::AccountIsNotAnAdmin.into());
    }

    let idx = dao
        .admins
        .iter()
        .position(|&x| x == ctx.accounts.account.key());
    dao.admins.remove(idx.unwrap());

    Ok(())
}
