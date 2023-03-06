use crate::schemas::{Dao, Proposal, Voter};
use crate::utils::constants::*;
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Just a pure account
    pub super_admin: AccountInfo<'info>,

    #[account(
        seeds = [
            DAO_PREFIX.as_ref(),
            dao.super_admin.key().as_ref()
        ],
        bump,
        constraint = dao.super_admin.key() == super_admin.key()
    )]
    pub dao: Account<'info, Dao>,

    #[account(
        mut,
        seeds = [
            PROPOSAL_PREFIX.as_ref(),
            authority.key().as_ref(),
            dao.num_of_proposals.to_string().as_bytes().as_ref(),
        ],
        bump,
        has_one = token_vote
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        mut,
        seeds = [
            VOTER_PREFIX.as_ref(),
            authority.key().as_ref(),
            proposal.key().as_ref()
        ],
        bump,
    )]
    pub voter: Account<'info, Voter>,

    #[account(
        constraint = token_vote.decimals == 0 && token_vote.supply > 0
    )]
    pub token_vote: Account<'info, token::Mint>,

    #[account(
        mut,
        associated_token::mint = token_vote,
        associated_token::authority = authority
    )]
    pub authority_token_account: Account<'info, token::TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_vote,
        associated_token::authority = proposal
    )]
    pub proposal_token_account: Account<'info, token::TokenAccount>,

    // System program addresses
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    pub time: Sysvar<'info, Clock>,
}

pub fn exec(ctx: Context<Claim>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let authority: &Signer = &ctx.accounts.authority;
    let voter = &mut ctx.accounts.voter;
    let authority_token_account = &mut ctx.accounts.authority_token_account;
    let proposal_token_account = &mut ctx.accounts.proposal_token_account;

    if voter.is_claimed {
        return Err(ErrorCode::IsAlreadyClaimed.into());
    }

    let now: u64 = ctx.accounts.time.unix_timestamp as u64;

    if now <= proposal.end_time {
        // return Err(ErrorCode::NowCannotClaim.into());
    }

    if voter.amount == 0 {
        return Err(ErrorCode::NothingToClaim.into());
    }

    voter.is_claimed = true;

    let num = ctx.accounts.dao.num_of_proposals.to_string();

    let seeds: &[&[&[u8]]] = &[&[
        PROPOSAL_PREFIX.as_ref(),
        &authority.key().to_bytes(),
        num.as_bytes().as_ref(),
        &[*ctx.bumps.get("proposal").unwrap()],
    ]];

    // Send Semi NFT

    let transfer_ctx: CpiContext<token::Transfer> = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: proposal_token_account.to_account_info(),
            to: authority_token_account.to_account_info(),
            authority: proposal.to_account_info(),
        },
        seeds,
    );
    token::transfer(transfer_ctx, voter.amount as u64)?;

    Ok(())
}
