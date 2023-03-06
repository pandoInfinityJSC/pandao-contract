use anchor_lang::prelude::*;

pub mod schemas;

pub mod utils;
pub use utils::*;

pub mod instructions;
pub use instructions::*;

declare_id!("6HzSgqLgUCFKSs48EPMWtuE5GznKTqMJQPxNMnqHNC41");

#[program]
pub mod hackathon {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::exec(ctx)
    }

    pub fn grant_admin(ctx: Context<SetAdmins>) -> Result<()> {
        set_admins::grant_admin(ctx)
    }

    pub fn revoke_admin(ctx: Context<SetAdmins>) -> Result<()> {
        set_admins::revoke_admin(ctx)
    }

    pub fn create_proposal(
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
        create_proposal::exec(
            ctx,
            title,
            description,
            start_time,
            end_time,
            vote_type,
            num_of_options,
            tokens_per_option,
            threshold,
            max_options_per_vote,
        )
    }

    pub fn vote(ctx: Context<Vote>, options: String) -> Result<()> {
        vote::exec(ctx, options)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        claim::exec(ctx)
    }
}
