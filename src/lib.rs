use anchor_lang::{declare_id, prelude::AccountMeta, AccountDeserialize};
use anchor_spl::token::spl_token::state::Account;
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
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, sysvar};
use state::{WooAmmPool, WooConfig, WooPool, Wooracle};
use std::cmp::max;
use util::{
    balance, checked_mul_div_round_up, get_price, swap_math, Decimals, GetStateResult
};

use jupiter_amm_interface::{
    try_get_account_data, AccountMap, Amm, AmmContext, ClockRef, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapParams
};

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

mod constants;
mod errors;
pub mod state;
pub mod util;

#[cfg(feature = "devnet")]
declare_id!("Es677W33uwrXLSqjV3rqcz5sftyarupdV3vDpQ9LXGow");

#[cfg(not(feature = "devnet"))]
declare_id!("WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb");

#[derive(Clone)]
pub struct WoofiSwap {
    pub key: Pubkey,
    pub label: String,
    pub usdc_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub program_id: Pubkey,

    pub wooconfig: Pubkey,
    pub token_a_wooracle: Pubkey,
    pub token_a_woopool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_a_feed_account: Pubkey,
    pub token_a_price_update: Pubkey,
    pub token_b_wooracle: Pubkey,
    pub token_b_woopool: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_b_feed_account: Pubkey,
    pub token_b_price_update: Pubkey,
    pub usdc_feed_account: Pubkey,
    pub usdc_price_update: Pubkey,
    pub usdc_woopool: Pubkey,
    pub usdc_vault: Pubkey,

    pub fee_rate: u16,
    pub wooconfig_state: Option<WooConfig>,
    pub decimals_a: Option<Decimals>,
    pub state_a: Option<GetStateResult>,
    pub token_a_balance: Option<u128>,
    pub woopool_a: Option<WooPool>,
    pub decimals_b: Option<Decimals>,
    pub state_b: Option<GetStateResult>,
    pub token_b_balance: Option<u128>,
    pub woopool_b: Option<WooPool>,
    pub usdc_balance: Option<u128>,
    pub clock_ref: ClockRef,
}

impl Amm for WoofiSwap {
    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        let program_id = id();

        let woo_amm_pool = &WooAmmPool::try_deserialize(&mut keyed_account.account.data.as_slice())?;

        let wooconfig = woo_amm_pool.wooconfig;
        let token_a_mint = woo_amm_pool.token_mint_a;
        let token_a_wooracle = woo_amm_pool.wooracle_a;
        let token_a_woopool = woo_amm_pool.woopool_a;
        let token_a_vault = woo_amm_pool.token_vault_a;
        let token_a_feed_account = woo_amm_pool.feed_account_a;
        let token_a_price_update = woo_amm_pool.price_update_a;
        
        let token_b_mint = woo_amm_pool.token_mint_b;
        let token_b_wooracle = woo_amm_pool.wooracle_b;
        let token_b_woopool = woo_amm_pool.woopool_b;
        let token_b_vault = woo_amm_pool.token_vault_b;
        let token_b_feed_account = woo_amm_pool.feed_account_b;
        let token_b_price_update = woo_amm_pool.price_update_b;

        let usdc_mint = woo_amm_pool.quote_token_mint;
        let usdc_price_update = woo_amm_pool.quote_price_update;
        let usdc_feed_account = woo_amm_pool.quote_feed_account;
        let usdc_woopool = woo_amm_pool.quote_woopool;
        let usdc_vault = woo_amm_pool.quote_vault;

        Ok(WoofiSwap {
            key: keyed_account.key,
            label: "WoofiSwap".into(),
            program_id,
            usdc_mint,
            token_a_mint,
            token_b_mint,
            wooconfig,
            token_a_wooracle,
            token_a_woopool,
            token_a_vault,
            token_a_feed_account,
            token_a_price_update,
            token_b_wooracle,
            token_b_woopool,
            token_b_vault,
            token_b_feed_account,
            token_b_price_update,
            usdc_price_update,
            usdc_feed_account,
            usdc_woopool,
            usdc_vault,
            wooconfig_state: None,
            fee_rate: 0,
            decimals_a: None,
            state_a: None,
            woopool_a: None,
            token_a_balance: None,
            decimals_b: None,
            state_b: None,
            woopool_b: None,
            token_b_balance: None,
            usdc_balance: None,
            clock_ref: amm_context.clock_ref.clone(),       
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
        vec![
            self.wooconfig,
            self.token_a_wooracle,
            self.token_a_woopool,
            self.token_a_price_update,
            self.token_a_vault,
            self.token_b_wooracle,
            self.token_b_woopool,
            self.token_b_price_update,
            self.token_b_vault,
            self.usdc_vault,
            sysvar::clock::ID,
        ]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        println!("start update:");

        let wooconfig_data = &mut try_get_account_data(account_map, &self.wooconfig)?;
        let wooconfig = WooConfig::try_deserialize(wooconfig_data)?;

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

        let usdc_woopool_data = &mut try_get_account_data(account_map, &self.usdc_woopool)?;
        let usdc_woopool = WooPool::try_deserialize(usdc_woopool_data)?;

        let quote_price_update_data =
            &mut try_get_account_data(account_map, &self.usdc_price_update)?;
        let quote_price_update = &mut PriceUpdateV2::try_deserialize(quote_price_update_data)?;

        let fee_rate: u16 = if self.token_a_mint == self.usdc_mint {
            token_b_woopool.fee_rate
        } else if self.token_b_mint == self.usdc_mint {
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
            &self.clock_ref,
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
            &self.clock_ref,
            token_b_wooracle,
            token_b_price_update,
            quote_price_update,
        )?;

        let get_token_balance = |token_vault, woopool| {
            try_get_account_data(account_map, token_vault)
                .ok()
                .and_then(|account_data| {
                    Account::unpack(account_data).ok()
                })
                .and_then(|token_account| {
                    if token_account.is_frozen() {
                        None
                    } else {
                        balance(woopool, token_account.amount as u128)
                    }
                })
        };

        self.wooconfig_state = Some(wooconfig);
        self.token_a_balance = get_token_balance(&self.token_a_vault, &token_a_woopool);
        self.token_b_balance = get_token_balance(&self.token_b_vault, &token_b_woopool);
        self.usdc_balance = get_token_balance(&self.usdc_vault, &usdc_woopool);

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
        let wooconfig = self.wooconfig_state.as_ref().context("Missing wooconfig")?;
        if wooconfig.paused {
            return Err(anyhow!("Woofi is paused"));
        }

        let (decimals_from,
            state_from,
            woopool_from,
            decimals_to,
            state_to,
            woopool_to,
            token_from_balance,
            token_to_balance,
            usdc_balance
        ) = {
            if self.token_a_mint == quote_params.input_mint {
                (
                    self.decimals_a.as_ref().context("Missing decimals_a")?,
                    self.state_a.as_ref().context("Missing state_a")?,
                    self.woopool_a.as_ref().context("Missing woopool_a")?,
                    self.decimals_b.as_ref().context("Missing decimals_b")?,
                    self.state_b.as_ref().context("Missing state_b")?,
                    self.woopool_b.as_ref().context("Missing woopool_b")?,
                    self.token_a_balance.context("Missing token_b_balance")?,
                    self.token_b_balance.context("Missing token_b_balance")?,
                    self.usdc_balance.context("Missing usdc_balance")?
                )
            } else {
                (
                    self.decimals_b.as_ref().context("Missing decimals_b")?,
                    self.state_b.as_ref().context("Missing state_b")?,
                    self.woopool_b.as_ref().context("Missing woopool_b")?,
                    self.decimals_a.as_ref().context("Missing decimals_a")?,
                    self.state_a.as_ref().context("Missing state_a")?,
                    self.woopool_a.as_ref().context("Missing woopool_a")?,
                    self.token_b_balance.context("Missing token_b_balance")?,
                    self.token_a_balance.context("Missing token_a_balance")?,
                    self.usdc_balance.context("Missing usdc_balance")?
                )
            }
        };

        let in_amount = quote_params.amount as u128;

        if in_amount <= woopool_from.min_swap_amount {
            return
                Ok(Quote {
                    fee_pct: self.fee_rate.into(),
                    in_amount: in_amount as u64,
                    out_amount: 0,
                    fee_amount: 0,
                    fee_mint: self.usdc_mint,
                    ..Quote::default()
                });
        }

        let woopool_from_after = token_from_balance.checked_add(in_amount).context("computation overflow")?;
        if  woopool_from_after > woopool_from.cap_bal {
            return Err(ErrorCode::BalanceCapExceeds.into());
        }

        let usdc_amount: u128 = 
            if quote_params.input_mint == self.usdc_mint {
                in_amount
            } else {
                let (_usdc_amount, _) = swap_math::calc_quote_amount_sell_base(
                    in_amount,
                    woopool_from,
                    decimals_from,
                    state_from,
                )?;

                _usdc_amount
            };

        let swap_fee = checked_mul_div_round_up(usdc_amount, self.fee_rate as u128, ONE_E5_U128)?;
        let usdc_amount_after_fee = usdc_amount
            .checked_sub(swap_fee)
            .ok_or(ErrorCode::MathOverflow)?;

        let check_usdc_amount = 
            // sell base
            if woopool_to.token_mint == self.usdc_mint {
                usdc_amount
            }
            // sell quote/ base to base 
            else {
                swap_fee
            };

        if usdc_balance < check_usdc_amount {
            return Err(ErrorCode::NotEnoughOut.into());
        }

        let to_amount: u128 = 
            if quote_params.output_mint == self.usdc_mint {
                usdc_amount_after_fee
            } else {
                let (_to_amount, _) = swap_math::calc_base_amount_sell_quote(
                    usdc_amount_after_fee,
                    woopool_to,
                    decimals_to,
                    state_to,
                )?;
                _to_amount
            };

        if token_to_balance < to_amount {
            return Err(ErrorCode::NotEnoughOut.into());
        }

        Ok(Quote {
            fee_pct: self.fee_rate.into(),
            in_amount: in_amount as u64,
            out_amount: to_amount as u64,
            fee_amount: swap_fee as u64,
            fee_mint: self.usdc_mint,
            ..Quote::default()
        })
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let account_metas = vec![
            AccountMeta::new(self.wooconfig, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new(swap_params.token_transfer_authority, true),
            AccountMeta::new(self.token_a_wooracle, false),
            AccountMeta::new(self.token_a_woopool, false),
            AccountMeta::new(swap_params.source_token_account, false),
            AccountMeta::new(self.token_a_vault, false),
            AccountMeta::new(self.token_a_price_update, false),
            AccountMeta::new(self.token_b_wooracle, false),
            AccountMeta::new(self.token_b_woopool, false),
            AccountMeta::new(swap_params.destination_token_account, false),
            AccountMeta::new(self.token_b_vault, false),
            AccountMeta::new(self.token_b_price_update, false),
            AccountMeta::new(self.usdc_woopool, false),
            AccountMeta::new(self.usdc_price_update, false),
            AccountMeta::new(self.usdc_vault, false),
            // TODO: confirm with Jupiter about the rebate address
            // AccountMeta::new(self.rebate_to, false),
        ];

        // TODO: uncomment this part in prod
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
