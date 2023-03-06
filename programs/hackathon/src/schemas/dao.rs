use anchor_lang::prelude::*;
use std::mem::size_of;

#[account]
pub struct Dao {
    pub super_admin: Pubkey,
    pub admins: Vec<Pubkey>,
    pub num_of_proposals: u16,
}

impl Dao {
    pub fn is_super_admin(&self, super_admin: Pubkey) -> bool {
        return self.super_admin == super_admin;
    }

    pub fn has_admin(&self, admin: Pubkey) -> bool {
        return self.admins.contains(&admin);
    }
}
