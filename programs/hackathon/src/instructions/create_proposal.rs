use crate::schemas::{Dao, Proposal};
use crate::utils::constants::*;
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            DAO_PREFIX.as_ref(),
            dao.super_admin.key().as_ref()
        ],
        bump,
        constraint = dao.has_admin(authority.key())
    )]
    pub dao: Account<'info, Dao>,

    #[account(
        init,
        payer = authority,
        space = PROPOSAL_SIZE,
        seeds = [
            PROPOSAL_PREFIX.as_ref(),
            authority.key().as_ref(),
            dao.num_of_proposals.to_string().as_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        constraint = token_vote.decimals == 0 && token_vote.supply > 0
    )]
    pub token_vote: Account<'info, token::Mint>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,

    pub time: Sysvar<'info, Clock>,
}

pub fn exec(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    start_time: u64,
    end_time: u64,
    vote_type: u8,
    num_of_options: u8,
    tokens_per_option: u16,
    threshold: u16,
    max_options_per_vote: u8,
) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let proposal = &mut ctx.accounts.proposal;
    let authority: &Signer = &ctx.accounts.authority;

    if title.chars().count() > PROPOSAL_MAX_TITLE {
        return Err(ErrorCode::InvalidTitleLength.into());
    }

    if description.chars().count() > PROPOSAL_MAX_DESCRIPTION {
        return Err(ErrorCode::InvalidDescriptionLength.into());
    }

    let now: u64 = ctx.accounts.time.unix_timestamp as u64;

    if start_time >= end_time || now > start_time {
        return Err(ErrorCode::InvalidTimestamp.into());
    }

    let vote_type: VoteType = VoteType::try_from(vote_type)?;

    if num_of_options == 0 {
        return Err(ErrorCode::InvalidNumOfOptions.into());
    }

    if tokens_per_option == 0 {
        return Err(ErrorCode::InvalidNumOfTokensPerOption.into());
    }

    if threshold == 0 || threshold > 1000 {
        return Err(ErrorCode::InvalidThreshold.into());
    }

    if vote_type == VoteType::Single && max_options_per_vote != 1
        || vote_type == VoteType::Polling
            && (max_options_per_vote < 1 || max_options_per_vote > num_of_options)
    {
        return Err(ErrorCode::InvalidMaxOptionsPerVote.into());
    }

    dao.num_of_proposals += 1;
    proposal.authority = authority.key();
    proposal.dao = dao.key();
    proposal.title = title;
    proposal.description = description;
    proposal.token_vote = ctx.accounts.token_vote.key();
    proposal.start_time = start_time;
    proposal.end_time = end_time;
    proposal.vote_type = vote_type as u8;
    proposal.num_of_options = num_of_options;
    proposal.num_of_nft_per_option = vec![0, num_of_options as u16];
    proposal.tokens_per_option = tokens_per_option;
    proposal.threshold = threshold;
    proposal.max_options_per_vote = max_options_per_vote;

    Ok(())
}
