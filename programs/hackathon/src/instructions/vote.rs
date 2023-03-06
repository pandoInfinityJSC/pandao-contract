use crate::schemas::{Dao, Proposal, Voter};
use crate::utils::constants::*;
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct Vote<'info> {
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
        init_if_needed,
        payer = authority,
        space = Voter::SIZE,
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
        init_if_needed,
        payer = authority,
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

pub fn exec(ctx: Context<Vote>, options: String) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let authority: &Signer = &ctx.accounts.authority;
    let voter = &mut ctx.accounts.voter;
    let authority_token_account = &mut ctx.accounts.authority_token_account;
    let proposal_token_account = &mut ctx.accounts.proposal_token_account;

    let options: Vec<u8> = options
        .split(",")
        .map(|o| o.trim().parse().unwrap())
        .collect();

    if options.len() == 0 || options.len() > proposal.max_options_per_vote as usize {
        return Err(ErrorCode::NumberOptionNotMatchProposalType.into());
    }

    let now: u64 = ctx.accounts.time.unix_timestamp as u64;

    if now < proposal.start_time || now > proposal.end_time {
        // return Err(ErrorCode::NowCannotVote.into());
    }

    let mut token_amount: u16 = 0;

    if voter.num_of_nft_per_voted.len() == 0 {
        voter.num_of_nft_per_voted = vec![0, proposal.num_of_options as u16]
    }

    for option in options.into_iter() {
        if option + 1 > proposal.num_of_options {
            return Err(ErrorCode::InvalidOption.into());
        }
        token_amount += proposal.tokens_per_option;
        proposal.num_of_nft_per_option[option as usize] += proposal.tokens_per_option;
        voter.amount += proposal.tokens_per_option as u64;
        voter.num_of_nft_per_voted[option as usize] += proposal.tokens_per_option;
    }

    // Send Semi NFT

    let transfer_ctx: CpiContext<token::Transfer> = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: authority_token_account.to_account_info(),
            to: proposal_token_account.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, token_amount as u64)?;

    Ok(())
}
