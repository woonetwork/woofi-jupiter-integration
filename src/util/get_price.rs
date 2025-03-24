use std::sync::atomic::Ordering;

use anchor_lang::{
    prelude::{borsh, AnchorDeserialize, AnchorSerialize},
    Key,
};
use anyhow::{Context, Result};
use jupiter_amm_interface::ClockRef;

use crate::{constants::*, errors::ErrorCode, state::wooracle::*};

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct GetPriceResult {
    pub price_out: u128,
    pub feasible_out: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct GetStateResult {
    pub price_out: u128,
    pub spread: u64,
    pub coeff: u64,
    pub feasible_out: bool,
}

pub fn get_price_impl<'info>(
    clock: &ClockRef,
    oracle: &Wooracle,
    price_update: &mut PriceUpdateV2,
    quote_price_update: &mut PriceUpdateV2,
) -> Result<GetPriceResult> {
    let now = clock.unix_timestamp.load(Ordering::Relaxed);

    let pyth_result = price_update
        .get_price_no_older_than(
            now,
            oracle.maximum_age,
            &oracle.feed_account.key().to_bytes(),
        )
        .ok()
        .context("pyth price update failed")?;

    let quote_price_result = quote_price_update
        .get_price_no_older_than(
            now,
            oracle.maximum_age,
            &oracle.quote_feed_account.key().to_bytes(),
        )
        .ok()
        .context("pyth price update failed")?;

    let base_price = pyth_result.price as u128;
    let quote_price = quote_price_result.price as u128;
    let quote_decimal = quote_price_result.exponent.abs() as u32;
    let clo_price = base_price
        .checked_mul(10_u128.pow(quote_decimal))
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(quote_price)
        .ok_or(ErrorCode::MathOverflow)?;

    let wo_price = oracle.price;
    let wo_timestamp = oracle.updated_at;
    let bound = oracle.bound as u128;

    let wo_feasible = clo_price != 0 && now <= (wo_timestamp + oracle.stale_duration);
    let wo_price_in_bound = clo_price != 0
        && ((clo_price * (ONE_E18_U128 - bound)) / ONE_E18_U128 <= wo_price
            && wo_price <= (clo_price * (ONE_E18_U128 + bound)) / ONE_E18_U128);

    let price_out: u128;
    let feasible_out: bool;
    if wo_feasible && wo_price_in_bound {
        price_out = wo_price;
        feasible_out = true;
    } else {
        price_out = 0;
        feasible_out = false;
    }

    if feasible_out {
        if price_out < oracle.range_min {
            return Err(ErrorCode::WooOraclePriceRangeMin.into());
        }
        if price_out > oracle.range_max {
            return Err(ErrorCode::WooOraclePriceRangeMax.into());
        }
    }

    Ok(GetPriceResult {
        price_out,
        feasible_out,
    })
}

pub fn get_state_impl<'info>(
    clock: &ClockRef,
    oracle: &Wooracle,
    price_update: &mut PriceUpdateV2,
    quote_price_update: &mut PriceUpdateV2,
) -> Result<GetStateResult> {
    let price_result = get_price_impl(clock, oracle, price_update, quote_price_update)?;
    Ok(GetStateResult {
        price_out: price_result.price_out,
        spread: oracle.spread,
        coeff: oracle.coeff,
        feasible_out: price_result.feasible_out,
    })
}
