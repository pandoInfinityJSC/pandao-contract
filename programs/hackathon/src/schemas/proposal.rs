use anchor_lang::prelude::*;
use std::mem::size_of;

#[account]
pub struct Proposal {
    pub authority: Pubkey,
    pub dao: Pubkey,
    pub title: String,
    pub description: String,
    pub token_vote: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub vote_type: u8, // 0: single choice, 1: polling
    pub num_of_options: u8,
    pub num_of_nft_per_option: Vec<u16>,
    pub tokens_per_option: u16,
    pub threshold: u16, // 600 = 60%
    pub max_options_per_vote: u8,
}
