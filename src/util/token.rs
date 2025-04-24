use anyhow::{anyhow, Context, Result};

use crate::errors::ErrorCode;

use crate::WooPool;

pub fn balance<'info>(
    woopool_state: &Option<WooPool>,
    token_vault_amount_state: Option<u64>,
) -> Result<u128> {
    let woopool = woopool_state.as_ref().context(anyhow!("Missing Woopool!"))?;
    let token_vault_amount = token_vault_amount_state.context(anyhow!("Missing token vault amount"))? as u128;

    let balance = if woopool.token_mint == woopool.quote_token_mint {
        token_vault_amount
            .checked_sub(woopool.unclaimed_fee)
            .ok_or(ErrorCode::ReserveLessThanFee)?
    } else {
        token_vault_amount
    };

    Ok(balance)
}