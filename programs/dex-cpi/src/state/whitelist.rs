use anchor_lang::prelude::*;
use crate::constants::MAX_WHITELIST_SIZE;
use crate::errors::ErrorCode;

#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub users: Vec<Pubkey>,
    pub paused: bool,
}

impl Whitelist {
    pub fn is_whitelisted(&self, user: &Pubkey) -> bool {
        self.users.contains(user)
    }

    pub fn add_user(&mut self, user: Pubkey) -> Result<()> {
        require!(self.users.len() < MAX_WHITELIST_SIZE, ErrorCode::WhitelistFull);
        if !self.is_whitelisted(&user) {
            self.users.push(user);
        }
        Ok(())
    }

    pub fn remove_user(&mut self, user: Pubkey) -> Result<()> {
        self.users.retain(|&u| u != user);
        Ok(())
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}