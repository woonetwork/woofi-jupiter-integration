use anchor_lang::prelude::*;

use crate::{constants::*, errors::ErrorCode, state::*, util::*};

pub fn calc_quote_amount_sell_base(
    base_amount: u128,
    woopool: &Account<'_, WooPool>,
    decimals: &Decimals,
    state: &GetStateResult,
) -> Result<(u128, u128)> {
    require!(state.feasible_out, ErrorCode::WooOracleNotFeasible);

    require!(state.price_out > 0, ErrorCode::WooOraclePriceNotValid);

    //let notionalSwap : u128 = (base_amount * state.price_out * decimals.quote_dec) / decimals.base_dec / decimals.price_dec;
    let notion_calc_a: u128 =
        checked_mul_div(base_amount, state.price_out, decimals.price_dec as u128)?;
    let notional_swap: u128 = checked_mul_div(
        notion_calc_a,
        decimals.quote_dec as u128,
        decimals.base_dec as u128,
    )?;

    require!(
        notional_swap <= woopool.max_notional_swap,
        ErrorCode::WooPoolExceedMaxNotionalValue
    );

    // gamma = k * price * base_amount; and decimal 18
    let gamma_calc_a: u128 =
        checked_mul_div(base_amount, state.price_out, decimals.price_dec as u128)?;
    let gamma: u128 =
        checked_mul_div(gamma_calc_a, state.coeff as u128, decimals.base_dec as u128)?;
    require!(gamma <= woopool.max_gamma, ErrorCode::WooPoolExceedMaxGamma);

    // Formula: quoteAmount = baseAmount * oracle.price * (1 - oracle.k * baseAmount * oracle.price - oracle.spread)
    // quoteAmount =
    // (((baseAmount * state.price * decs.quoteDec) / decs.priceDec) *
    //     (uint256(1e18) - gamma - state.spread)) /
    // 1e18 /
    // decs.baseDec;
    // ====>
    // quoteAmount =
    // (((baseAmount * state.price / decs.priceDec) * decs.quoteDec) * (uint256(1e18) - gamma - state.spread)) /
    // 1e18 /
    // decs.baseDec;
    // ====>
    // a = (baseAmount * state.price / decs.priceDec)
    // b = (uint256(1e18) - gamma - state.spread)
    // quoteAmount = ((a * decs.quoteDec) * b) / 1e18 / decs.baseDec;
    //             = ((a * b) / 1e18) * decs.quoteDec / decs.baseDec

    let calc_a: u128 = checked_mul_div(base_amount, state.price_out, decimals.price_dec as u128)?;
    let calc_b: u128 = ONE_E18_U128
        .checked_sub(gamma).ok_or(ErrorCode::MathOverflow)?
        .checked_sub(state.spread as u128).ok_or(ErrorCode::MathOverflow)?;
    let calc_c = checked_mul_div(calc_a, calc_b, ONE_E18_U128)?;
    let quote_amount = checked_mul_div(
        calc_c,
        decimals.quote_dec as u128,
        decimals.base_dec as u128,
    )?;

    // newPrice = oracle.price * (1 - k * oracle.price * baseAmount)
    let new_price: u128 = checked_mul_div(
        ONE_E18_U128.checked_sub(gamma).ok_or(ErrorCode::MathOverflow)?,
        state.price_out,
        ONE_E18_U128,
    )?;

    Ok((quote_amount, new_price))
}

pub fn calc_base_amount_sell_quote(
    quote_amount: u128,
    woopool: &Account<'_, WooPool>,
    decimals: &Decimals,
    state: &GetStateResult,
) -> Result<(u128, u128)> {
    require!(state.feasible_out, ErrorCode::WooOracleNotFeasible);

    require!(state.price_out > 0, ErrorCode::WooOraclePriceNotValid);

    require!(
        quote_amount <= woopool.max_notional_swap,
        ErrorCode::WooPoolExceedMaxNotionalValue
    );

    // gamma = k * quote_amount; and decimal 18
    let gamma: u128 = checked_mul_div(
        quote_amount,
        state.coeff as u128,
        decimals.quote_dec as u128,
    )?;
    require!(gamma <= woopool.max_gamma, ErrorCode::WooPoolExceedMaxGamma);

    // Formula: baseAmount = quoteAmount / oracle.price * (1 - oracle.k * quoteAmount - oracle.spread)
    let calc_a: u128 = quote_amount
        .checked_mul(decimals.base_dec as u128)
        .ok_or(ErrorCode::MathOverflow)?;
    let calc_b: u128 = checked_mul_div(calc_a, decimals.price_dec as u128, state.price_out)?;
    let calc_c: u128 = ONE_E18_U128
        .checked_sub(gamma).ok_or(ErrorCode::MathOverflow)?
        .checked_sub(state.spread as u128).ok_or(ErrorCode::MathOverflow)?;
    let calc_d: u128 = checked_mul_div(calc_b, calc_c, ONE_E18_U128)?;
    let base_amount = calc_d.checked_div(decimals.quote_dec as u128).ok_or(ErrorCode::MathOverflow)?;

    // new_price = oracle.price / (1 - k * quoteAmount)
    let new_price: u128 = checked_mul_div(
        ONE_E18_U128,
        state.price_out,
        ONE_E18_U128.checked_sub(gamma).ok_or(ErrorCode::MathOverflow)?,
    )?;

    Ok((base_amount, new_price))
}
