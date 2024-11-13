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

use crate::{constants::*, errors::ErrorCode};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WooPool {
    pub wooconfig: Pubkey,     // 32
    pub woopool_bump: [u8; 1], // 1
    pub authority: Pubkey,     // 32
    pub wooracle: Pubkey,      // 32

    // 1 in 100000; 10 = 1bp = 0.01%; max = 65535
    pub fee_rate: u16, // 2
    // max range of `balance * k`
    pub max_gamma: u128, // 16
    // max volume per swap
    pub max_notional_swap: u128, // 16
    // max balance cap in token amount
    pub cap_bal: u128, // 16
    // min from amount when swap
    pub min_swap_amount: u128, // 16

    pub unclaimed_fee: u128,      // 16
    pub token_mint: Pubkey,       // 32
    pub token_vault: Pubkey,      // 32
    pub quote_token_mint: Pubkey, // 32
    pub base_decimals: u8,        // 1
}

impl WooPool {
    pub fn seeds(&self) -> [&[u8]; 5] {
        [
            WOOPOOL_SEED.as_bytes(),
            self.wooconfig.as_ref(),
            self.token_mint.as_ref(),
            self.quote_token_mint.as_ref(),
            self.woopool_bump.as_ref(),
        ]
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        &mut self,
        bump: u8,
        wooconfig: Pubkey,
        authority: Pubkey,
        wooracle: Pubkey,
        token_mint: Pubkey,
        token_vault: Pubkey,
        quote_token_mint: Pubkey,
        base_decimals: u8,
    ) -> Result<()> {
        self.woopool_bump = [bump];
        self.wooconfig = wooconfig;
        self.authority = authority;
        self.wooracle = wooracle;

        self.fee_rate = 0;
        self.unclaimed_fee = 0;
        self.max_gamma = 0;
        self.max_notional_swap = 0;
        self.cap_bal = 0;
        self.min_swap_amount = 0;

        self.token_mint = token_mint;
        self.token_vault = token_vault;

        self.quote_token_mint = quote_token_mint;

        self.base_decimals = base_decimals;

        Ok(())
    }

    pub fn set_fee_rate(&mut self, fee_rate: u16) -> Result<()> {
        if fee_rate > MAX_FEE_RATE {
            return Err(ErrorCode::FeeRateMaxExceeded.into());
        }
        self.fee_rate = fee_rate;

        Ok(())
    }

    pub fn set_max_gamma(&mut self, max_gamma: u128) -> Result<()> {
        self.max_gamma = max_gamma;

        Ok(())
    }

    pub fn set_max_notional_swap(&mut self, max_notional_swap: u128) -> Result<()> {
        self.max_notional_swap = max_notional_swap;

        Ok(())
    }

    pub fn set_cap_bal(&mut self, cap_bal: u128) -> Result<()> {
        self.cap_bal = cap_bal;

        Ok(())
    }

    pub fn set_min_swap_amount(&mut self, min_swap_amount: u128) -> Result<()> {
        self.min_swap_amount = min_swap_amount;

        Ok(())
    }

    pub fn add_unclaimed_fee(&mut self, fee: u128) -> Result<()> {
        self.unclaimed_fee = self
            .unclaimed_fee
            .checked_add(fee)
            .ok_or(ErrorCode::ProtocolFeeMaxExceeded)?;

        Ok(())
    }

    pub fn sub_unclaimed_fee(&mut self, fee: u128) -> Result<()> {
        if fee > self.unclaimed_fee {
            return Err(ErrorCode::ProtocolFeeNotEnough.into());
        }

        self.unclaimed_fee -= fee;

        Ok(())
    }
}
