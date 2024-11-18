use anchor_lang::prelude::*;

use crate::errors::ErrorCode;

pub const ADMIN_AUTH_MAX_LEN: usize = 5;
pub const FEE_AUTH_MAX_LEN: usize = 5;
pub const GUARDIAN_AUTH_MAX_LEN: usize = 5;
pub const PAUSE_AUTH_MAX_LEN: usize = 5;

#[account]
#[derive(InitSpace)]
pub struct WooConfig {
    pub authority: Pubkey,

    pub paused: bool,

    #[max_len(ADMIN_AUTH_MAX_LEN)]
    pub woopool_admin_authority: Vec<Pubkey>,

    #[max_len(ADMIN_AUTH_MAX_LEN)]
    pub wooracle_admin_authority: Vec<Pubkey>,

    #[max_len(FEE_AUTH_MAX_LEN)]
    pub fee_authority: Vec<Pubkey>,

    #[max_len(GUARDIAN_AUTH_MAX_LEN)]
    pub guardian_authority: Vec<Pubkey>,

    #[max_len(PAUSE_AUTH_MAX_LEN)]
    pub pause_authority: Vec<Pubkey>,

    pub new_authority: Pubkey,
}