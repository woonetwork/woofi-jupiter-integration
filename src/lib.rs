use anchor_lang::{declare_id, prelude::AccountMeta, AccountDeserialize};
/*

░██╗░░░░░░░██╗░█████╗░░█████╗░░░░░░░███████╗██╗
░██║░░██╗░░██║██╔══██╗██╔══██╗░░░░░░██╔════╝██║
░╚██╗████╗██╔╝██║░░██║██║░░██║█████╗█████╗░░██║
░░████╔═████║░██║░░██║██║░░██║╚════╝██╔══╝░░██║
░░╚██╔╝░╚██╔╝░╚█████╔╝╚█████╔╝░░░░░░██║░░░░░██║
░░░╚═╝░░░╚═╝░░░╚════╝░░╚════╝░░░░░░░╚═╝░░░░░╚═╝

*
* MIT License
* ===========
*
* Copyright (c) 2020 WooTrade
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use anyhow::{Context, Result};

use constants::ONE_E5_U128;
use errors::ErrorCode;
use solana_sdk::{clock::Clock, pubkey::Pubkey, sysvar};
use state::{WooPool, Wooracle};
use std::{cmp::max, convert::TryInto};
use util::{
    checked_mul_div_round_up, get_price, get_pubkey_from_param, get_wooconfig_address,
    get_woopool_address, get_wooracle_address, swap_math, Decimals, GetStateResult, SOL, USDC,
};

use jupiter_amm_interface::{
    try_get_account_data, AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams,
    SwapAndAccountMetas, SwapParams,
};

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

mod constants;
mod errors;
pub mod state;
pub mod util;

#[cfg(feature = "devnet")]
declare_id!("Es677W33uwrXLSqjV3rqcz5sftyarupdV3vDpQ9LXGow");

#[cfg(not(feature = "devnet"))]
declare_id!("WooFiGFK9x5FBYdvMg3pJBpAkPA8EEYiLcopwrxJvDG");

#[derive(Clone)]
pub struct WoofiSwap {
    pub key: Pubkey,
    pub label: String,
    pub quote_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub program_id: Pubkey,

    pub wooconfig: Pubkey,
    pub token_a_wooracle: Pubkey,
    pub token_a_woopool: Pubkey,
    pub token_a_feed_account: Pubkey,
    pub token_a_price_update: Pubkey,
    pub token_b_wooracle: Pubkey,
    pub token_b_woopool: Pubkey,
    pub token_b_feed_account: Pubkey,
    pub token_b_price_update: Pubkey,
    pub quote_pool: Pubkey,
    pub quote_price_update: Pubkey,

    pub fee_rate: u16,
    pub decimals_a: Option<Decimals>,
    pub state_a: Option<GetStateResult>,
    pub woopool_a: Option<WooPool>,
    pub decimals_b: Option<Decimals>,
    pub state_b: Option<GetStateResult>,
    pub woopool_b: Option<WooPool>,
}

impl Amm for WoofiSwap {
    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        let program_id = id();

        let woopool = &WooPool::try_deserialize(&mut keyed_account.account.data.as_slice())?;

        let quote_mint = woopool.quote_token_mint;
        let token_a_mint = woopool.token_mint;
        let token_b_mint = woopool.quote_token_mint;

        let params = keyed_account
            .params
            .as_ref()
            .context("missing keyed account params")?;
        let param_map = params
            .as_object()
            .context("keyed account params is not correct")?;
        let token_a_feed_account =
            get_pubkey_from_param(param_map, "token_a_feed_account".to_string())?;
        let token_a_price_update =
            get_pubkey_from_param(param_map, "token_a_price_update".to_string())?;
        let token_b_feed_account =
            get_pubkey_from_param(param_map, "token_b_feed_account".to_string())?;
        let token_b_price_update =
            get_pubkey_from_param(param_map, "token_b_price_update".to_string())?;
        let quote_price_update =
            get_pubkey_from_param(param_map, "quote_price_update".to_string())?;
        let wooconfig = get_wooconfig_address(&program_id).0;
        let token_a_wooracle = get_wooracle_address(
            &wooconfig,
            &token_a_mint,
            &token_a_feed_account,
            &token_a_price_update,
            &program_id,
        )
        .0;
        let token_b_wooracle = get_wooracle_address(
            &wooconfig,
            &token_b_mint,
            &token_b_feed_account,
            &token_b_price_update,
            &program_id,
        )
        .0;
        let token_a_woopool =
            get_woopool_address(&wooconfig, &token_a_mint, &quote_mint, &program_id).0;
        let token_b_woopool =
            get_woopool_address(&wooconfig, &token_b_mint, &quote_mint, &program_id).0;
        let quote_pool = get_woopool_address(&wooconfig, &quote_mint, &quote_mint, &program_id).0;
        Ok(WoofiSwap {
            key: keyed_account.key,
            label: "WoofiSwap".into(),
            program_id,
            quote_mint,
            token_a_mint,
            token_b_mint,
            wooconfig,
            token_a_wooracle,
            token_a_woopool,
            token_a_feed_account,
            token_a_price_update,
            token_b_wooracle,
            token_b_woopool,
            token_b_feed_account,
            token_b_price_update,
            quote_price_update,
            quote_pool,
            fee_rate: 0,
            decimals_a: None,
            state_a: None,
            woopool_a: None,
            decimals_b: None,
            state_b: None,
            woopool_b: None,
        })
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn key(&self) -> Pubkey {
        self.key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.token_a_mint, self.token_b_mint]
    }

    fn has_dynamic_accounts(&self) -> bool {
        true
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![
            self.token_a_wooracle,
            self.token_a_woopool,
            self.token_a_price_update,
            self.token_b_wooracle,
            self.token_b_woopool,
            self.token_b_price_update,
            sysvar::clock::ID,
        ]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        println!("start update:");

        let token_a_wooracle_data = &mut try_get_account_data(account_map, &self.token_a_wooracle)?;
        let token_a_wooracle = &Wooracle::try_deserialize(token_a_wooracle_data)?;

        let token_a_woopool_data = &mut try_get_account_data(account_map, &self.token_a_woopool)?;
        let token_a_woopool = WooPool::try_deserialize(token_a_woopool_data)?;

        let token_a_price_update_data =
            &mut try_get_account_data(account_map, &self.token_a_price_update)?;
        let token_a_price_update = &mut PriceUpdateV2::try_deserialize(token_a_price_update_data)?;

        let token_b_wooracle_data = &mut try_get_account_data(account_map, &self.token_b_wooracle)?;
        let token_b_wooracle = &Wooracle::try_deserialize(token_b_wooracle_data)?;

        let token_b_woopool_data = &mut try_get_account_data(account_map, &self.token_b_woopool)?;
        let token_b_woopool = WooPool::try_deserialize(token_b_woopool_data)?;

        let token_b_price_update_data =
            &mut try_get_account_data(account_map, &self.token_b_price_update)?;
        let token_b_price_update = &mut PriceUpdateV2::try_deserialize(token_b_price_update_data)?;

        let quote_price_update_data =
            &mut try_get_account_data(account_map, &self.quote_price_update)?;
        let quote_price_update = &mut PriceUpdateV2::try_deserialize(quote_price_update_data)?;

        let clock: Clock = match account_map.get(&sysvar::clock::ID) {
            Some(account) => bincode::deserialize(&account.data)
                .context("Failed to deserialize sysvar::clock::ID")?,
            None => Clock::default(), // some amms don't have clock snapshot
        };

        let fee_rate: u16 = if self.token_a_mint == self.quote_mint {
            token_b_woopool.fee_rate
        } else if self.token_b_mint == self.quote_mint {
            token_a_woopool.fee_rate
        } else {
            max(token_a_woopool.fee_rate, token_b_woopool.fee_rate)
        };

        let decimals_a = Decimals::new(
            token_a_wooracle.price_decimals as u32,
            token_a_wooracle.quote_decimals as u32,
            token_a_wooracle.base_decimals as u32,
        );

        let state_a = get_price::get_state_impl(
            &clock,
            token_a_wooracle,
            token_a_price_update,
            quote_price_update,
        )?;

        let decimals_b = Decimals::new(
            token_b_wooracle.price_decimals as u32,
            token_b_wooracle.quote_decimals as u32,
            token_b_wooracle.base_decimals as u32,
        );

        let state_b = get_price::get_state_impl(
            &clock,
            token_b_wooracle,
            token_b_price_update,
            quote_price_update,
        )?;

        self.fee_rate = fee_rate;
        self.decimals_a = decimals_a;
        self.state_a = Some(state_a);
        self.woopool_a = Some(token_a_woopool);
        self.decimals_b = decimals_b;
        self.state_b = Some(state_b);
        self.woopool_b = Some(token_b_woopool);

        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let (decimals_a, state_a, woopool_a, decimals_b, state_b, woopool_b) = {
            if self.token_a_mint == quote_params.input_mint {
                (
                    self.decimals_a,
                    self.state_a,
                    self.woopool_a,
                    self.decimals_b,
                    self.state_b,
                    self.woopool_b,
                )
            } else {
                (
                    self.decimals_b,
                    self.state_b,
                    self.woopool_b,
                    self.decimals_a,
                    self.state_a,
                    self.woopool_a,
                )
            }
        };

        let mut quote_amount = quote_params.amount as u128;
        if quote_params.input_mint != self.quote_mint {
            let (_quote_amount, _) = swap_math::calc_quote_amount_sell_base(
                quote_params.amount as u128,
                woopool_a.as_ref().context("Missing woopool")?,
                decimals_a.as_ref().context("Missing decimals_a")?,
                state_a.as_ref().context("Missing state_a")?,
            )?;

            quote_amount = _quote_amount;
        }

        let swap_fee = checked_mul_div_round_up(quote_amount, self.fee_rate as u128, ONE_E5_U128)?;
        quote_amount = quote_amount
            .checked_sub(swap_fee)
            .ok_or(ErrorCode::MathOverflow)?;

        let mut to_amount = quote_amount;
        if quote_params.output_mint != self.quote_mint {
            let (_to_amount, _) = swap_math::calc_base_amount_sell_quote(
                quote_amount,
                woopool_b.as_ref().context("Missing woopool")?,
                decimals_b.as_ref().context("Missing decimals_a")?,
                state_b.as_ref().context("Missing state_a")?,
            )?;
            to_amount = _to_amount;
        }

        Ok(Quote {
            fee_pct: self.fee_rate.into(),
            in_amount: quote_params.amount.try_into()?,
            out_amount: to_amount as u64,
            fee_amount: swap_fee as u64,
            fee_mint: self.quote_mint,
            ..Quote::default()
        })
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let woopool_a = self.woopool_a.ok_or(ErrorCode::SwapPoolInvalid)?;
        let woopool_b = self.woopool_b.ok_or(ErrorCode::SwapPoolInvalid)?;
        let quote_vault = woopool_b.token_vault;

        let account_metas = vec![
            AccountMeta::new(self.wooconfig, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new(swap_params.token_transfer_authority, true),
            AccountMeta::new(self.token_a_wooracle, false),
            AccountMeta::new(self.token_a_woopool, false),
            AccountMeta::new(swap_params.source_token_account, false),
            AccountMeta::new(woopool_a.token_vault, false),
            AccountMeta::new(self.token_a_price_update, false),
            AccountMeta::new(self.token_b_wooracle, false),
            AccountMeta::new(self.token_b_woopool, false),
            AccountMeta::new(swap_params.destination_token_account, false),
            AccountMeta::new(woopool_b.token_vault, false),
            AccountMeta::new(self.token_b_price_update, false),
            AccountMeta::new(self.quote_pool, false),
            AccountMeta::new(self.quote_price_update, false),
            AccountMeta::new(quote_vault, false),
            //            AccountMeta::new(self.rebate_to, false),
        ];

        // Ok(SwapAndAccountMetas {
        //     swap: Swap::WoofiSwap,
        //     account_metas
        // })
        unimplemented!()
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}
