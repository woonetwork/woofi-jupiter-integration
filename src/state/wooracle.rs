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

use anchor_lang::prelude::{account, borsh, AnchorDeserialize, AnchorSerialize, InitSpace, Pubkey};
use anchor_lang::Discriminator;
#[account]
#[derive(InitSpace)]
pub struct Wooracle {
    pub wooconfig: Pubkey,  // 32
    pub authority: Pubkey,  // 32
    pub token_mint: Pubkey, // 32

    // pyth feed account for BASE token
    pub feed_account: Pubkey, // 32
    // pyth price update account for BASE token
    pub price_update: Pubkey, // 32
    // pyth oracle maximum age, in seconds, 60 means 60s
    pub maximum_age: u64, // 8

    pub price_decimals: u8,  // 1
    pub quote_decimals: u8,  // 1
    pub base_decimals: u8,   // 1
    pub updated_at: i64,     // 8
    pub stale_duration: i64, // 8
    pub bound: u64,          // 8
    pub price: u128,         // 16 as chainlink oracle (e.g. decimal = 8)
    pub coeff: u64,          // 8 k: decimal = 18.    18.4 * 1e18
    pub spread: u64,         // 8 s: decimal = 18.   spread <= 2e18   18.4 * 1e18
    pub range_min: u128,     // 16
    pub range_max: u128,     // 16

    // quote token configs
    pub quote_token_mint: Pubkey,   // 32
    pub quote_feed_account: Pubkey, // 32
    pub quote_price_update: Pubkey, // 32
}
