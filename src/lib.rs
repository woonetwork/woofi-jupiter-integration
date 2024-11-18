use anchor_lang::{declare_id, prelude::Account, AccountDeserialize};
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
use anyhow::{anyhow, Context, Result};

use constants::ONE_E5_U128;
use errors::ErrorCode;
use state::{wooconfig, woopool, WooPool, Wooracle};
use util::{checked_mul_div_round_up, decimals, get_price, get_wooconfig_address, get_woopool_address, get_wooracle_address, swap_math, Decimals, GetPriceResult, GetStateResult, SOL, SOL_FEED_ACCOUNT, SOL_PRICE_UPDATE, USDC, USDC_FEED_ACCOUNT, USDC_PRICE_UPDATE};
use std::{cmp::max, collections::HashMap, convert::TryInto};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

use jupiter_amm_interface::{
    try_get_account_data, AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas, SwapMode, SwapParams
};

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

mod constants;
mod errors;
mod state;
pub mod util;

declare_id!("woocYbcZkJ1ryopvtNP7Lr367wbW4WrMgxmzroe6VWU");

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
    pub quote_price_update: Pubkey,

    pub fee_rate: u16,
    pub decimals_a: Decimals,
    pub state_a: GetStateResult,
    pub woopool_a: Option<WooPool>,
    pub decimals_b: Decimals,
    pub state_b: GetStateResult,
    pub woopool_b: Option<WooPool>
}

impl WoofiSwap {
    fn get_authority(&self) -> Pubkey {
        Pubkey::find_program_address(&[&self.key.to_bytes()], &self.program_id).0
    }
}

impl Amm for WoofiSwap {
    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        let program_id = keyed_account.key;
        let quote_mint = USDC;
        let token_a_mint = SOL;
        let token_b_mint = USDC;
        let token_a_feed_account = SOL_FEED_ACCOUNT;
        let token_a_price_update = SOL_PRICE_UPDATE;
        let token_b_feed_account = USDC_FEED_ACCOUNT;
        let token_b_price_update = USDC_PRICE_UPDATE;
        let quote_price_update = USDC_PRICE_UPDATE;
        let wooconfig = get_wooconfig_address(&program_id).0;
        let token_a_wooracle = get_wooracle_address(&wooconfig, &token_a_mint, &token_a_feed_account, &token_a_price_update, &program_id).0;
        let token_b_wooracle = get_wooracle_address(&wooconfig, &token_b_mint, &token_b_feed_account, &token_b_price_update, &program_id).0;
        let token_a_woopool = get_woopool_address(&wooconfig, &token_a_mint, &quote_mint, &program_id).0;
        let token_b_woopool = get_woopool_address(&wooconfig, &token_b_mint, &quote_mint, &program_id).0;

        let init_decimals = Decimals {
            price_dec: 0,
            quote_dec: 0,
            base_dec: 0
        };
        let init_state_result = GetStateResult {
            price_out: 0,
            spread: 0,
            coeff: 0,
             feasible_out: false
        };

        Ok(WoofiSwap {
            key: program_id,
            label: "WoofiSwap".into(),
            quote_mint,
            token_a_mint,
            token_b_mint,
            program_id,
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
            fee_rate: 0,
            decimals_a: init_decimals,
            state_a: init_state_result,
            woopool_a: None,
            decimals_b: init_decimals,
            state_b: init_state_result,
            woopool_b: None
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
    
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.token_a_wooracle, self.token_a_woopool, self.token_a_price_update,
             self.token_b_wooracle, self.token_b_woopool, self.token_b_price_update
            ]
    }
    
    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let token_a_wooracle_data = &mut try_get_account_data(account_map, &self.token_a_wooracle)?;
        let token_a_wooracle = & Wooracle::try_deserialize( token_a_wooracle_data)?;

        let token_a_woopool_data = &mut try_get_account_data(account_map, &self.token_a_woopool)?;
        let token_a_woopool = WooPool::try_deserialize(token_a_woopool_data)?;

        let token_a_price_update_data = &mut try_get_account_data(account_map, &self.token_a_price_update)?;
        let token_a_price_update = &mut PriceUpdateV2::try_deserialize(token_a_price_update_data)?;

        let token_b_wooracle_data = &mut try_get_account_data(account_map, &self.token_b_wooracle)?;
        let token_b_wooracle = & Wooracle::try_deserialize(token_b_wooracle_data)?;

        let token_b_woopool_data = &mut try_get_account_data(account_map, &self.token_b_woopool)?;
        let token_b_woopool = WooPool::try_deserialize(token_b_woopool_data)?;
    
        let token_b_price_update_data = &mut try_get_account_data(account_map, &self.token_b_price_update)?;
        let token_b_price_update = &mut PriceUpdateV2::try_deserialize(token_b_price_update_data)?;

        let quote_price_update_data = &mut try_get_account_data(account_map, &self.quote_price_update)?;
        let quote_price_update = &mut PriceUpdateV2::try_deserialize(quote_price_update_data)?;

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
        let state_a =
            get_price::get_state_impl(token_a_wooracle, token_a_price_update, quote_price_update)?;

        let decimals_b = Decimals::new(
            token_b_wooracle.price_decimals as u32,
            token_b_wooracle.quote_decimals as u32,
            token_b_wooracle.base_decimals as u32,
        );
        let state_b =
            get_price::get_state_impl(token_b_wooracle, token_b_price_update, quote_price_update)?;    

        self.fee_rate = fee_rate;
        self.decimals_a = decimals_a?;
        self.state_a = state_a;
        self.woopool_a = Some(token_a_woopool);
        self.decimals_b = decimals_b?;
        self.state_b = state_b;
        self.woopool_b = Some(token_b_woopool);

        Ok(())
    }
    
    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        if quote_params.swap_mode == SwapMode::ExactIn {
            let mut decimals_a = &self.decimals_a;
            let mut state_a = &self.state_a;
            let mut woopool_a = &self.woopool_a.clone().ok_or(ErrorCode::WooOracleNotFeasible)?;
            let mut decimals_b = &self.decimals_b;
            let mut state_b = &self.state_b;
            let mut woopool_b = &self.woopool_b.clone().ok_or(ErrorCode::WooOracleNotFeasible)?;
            if self.token_b_mint == quote_params.input_mint {
                decimals_a = &self.decimals_b;
                decimals_b = &self.decimals_a;
                state_a = &self.state_b;
                state_b = &self.state_a;
                let woopool_c = woopool_a;
                woopool_a = woopool_b;
                woopool_b = woopool_c;
            }

            let mut quote_amount = quote_params.amount as u128;
            if quote_params.input_mint != self.quote_mint {
        
                let (_quote_amount, _) = swap_math::calc_quote_amount_sell_base(
                    quote_params.amount as u128,
                    woopool_a,
                    decimals_a,
                    state_a,
                )?;
        
                quote_amount = _quote_amount;
            }
        
            let swap_fee = checked_mul_div_round_up(quote_amount, self.fee_rate as u128, ONE_E5_U128)?;
            quote_amount = quote_amount.checked_sub(swap_fee).ok_or(ErrorCode::MathOverflow)?;
        
            let mut to_amount = quote_amount;
            if quote_params.output_mint != self.quote_mint {
                let (_to_amount, _) = swap_math::calc_base_amount_sell_quote(
                    quote_amount,
                    woopool_b,
                    decimals_b,
                    state_b,
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
        } else {
            Ok(Quote {
                fee_pct: self.fee_rate.into(),
                in_amount: 0,
                out_amount: 0,
                fee_amount: 0,
                fee_mint: self.quote_mint,
                ..Quote::default()
            })

        }
    }
    
    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        todo!()
    }
    
    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        //Box::new(self.clone())
        todo!()
    }
    
}