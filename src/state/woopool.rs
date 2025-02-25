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

use anchor_lang::prelude::{account, AnchorSerialize, AnchorDeserialize, borsh, Pubkey, InitSpace};


#[account]
#[derive(InitSpace)]
#[derive(Default, Copy)]
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
