use anyhow::{anyhow, Context, Result};

use crate::errors::ErrorCode;

use crate::WooPool;

pub fn balance<'info>(
    woopool: &WooPool,
    token_vault_amount: u128,
) -> Option<u128> {
    if woopool.token_mint == woopool.quote_token_mint {
        token_vault_amount.checked_sub(woopool.unclaimed_fee)
    } else {
        Some(token_vault_amount)
    }
}