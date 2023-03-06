use anchor_lang::prelude::*;
use std::mem::size_of;

#[account]
pub struct Voter {
    pub authority: Pubkey,
    pub proposal: Pubkey,
    pub amount: u64,
    pub num_of_nft_per_voted: Vec<u16>,
    pub is_claimed: bool,
}

impl Voter {
    pub const SIZE: usize = size_of::<Voter>() + 8;
}
