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

use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WooAmmPool {
    pub wooconfig: Pubkey,
    pub wooswap_bump: [u8; 1],
    pub authority: Pubkey,
    pub wooracle_a: Pubkey,
    pub woopool_a: Pubkey,
    pub feed_account_a: Pubkey,
    pub price_update_a: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub wooracle_b: Pubkey,
    pub woopool_b: Pubkey,
    pub feed_account_b: Pubkey,
    pub price_update_b: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub quote_token_mint: Pubkey,
    pub quote_feed_account: Pubkey,
    pub quote_price_update: Pubkey,
}

impl WooAmmPool {
    pub fn seeds(&self) -> [&[u8]; 5] {
        [
            WOOAMMPOOL_SEED.as_bytes(),
            self.wooconfig.as_ref(),
            self.token_mint_a.as_ref(),
            self.token_mint_b.as_ref(),
            self.wooswap_bump.as_ref(),
        ]
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        &mut self,
        bump: u8,
        wooconfig: Pubkey,
        authority: Pubkey,
        wooracle_a: Pubkey,
        woopool_a: Pubkey,
        feed_account_a: Pubkey,
        price_update_a: Pubkey,
        token_mint_a: Pubkey,
        token_vault_a: Pubkey,
        wooracle_b: Pubkey,
        woopool_b: Pubkey,
        feed_account_b: Pubkey,
        price_update_b: Pubkey,
        token_mint_b: Pubkey,
        token_vault_b: Pubkey,
        quote_token_mint: Pubkey,
        quote_feed_account: Pubkey,
        quote_price_update: Pubkey,    
    ) -> Result<()> {
        self.wooswap_bump = [bump];
        self.wooconfig = wooconfig;
        self.authority = authority;
        self.wooracle_a = wooracle_a;
        self.woopool_a = woopool_a;
        self.feed_account_a = feed_account_a;
        self.price_update_a = price_update_a;
        self.token_mint_a = token_mint_a;
        self.token_vault_a = token_vault_a;
        self.wooracle_b = wooracle_b;
        self.woopool_b = woopool_b;
        self.feed_account_b = feed_account_b;
        self.price_update_b = price_update_b;
        self.token_mint_b = token_mint_b;
        self.token_vault_b = token_vault_b;
        self.quote_token_mint = quote_token_mint;
        self.quote_feed_account = quote_feed_account;
        self.quote_price_update = quote_price_update;

        Ok(())
    }
}
