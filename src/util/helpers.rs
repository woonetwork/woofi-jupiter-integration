use std::str::FromStr;

use anyhow::{Context, Result};

use serde_json::{Map, Value};
use solana_program::pubkey;
use solana_sdk::pubkey::Pubkey;


// PROD
pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub fn get_wooconfig_address(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooconfig"], program_id)
}

pub fn get_wooracle_address(wooconfig: &Pubkey, token_mint: &Pubkey, feed_account: &Pubkey, price_update: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooracle", wooconfig.as_ref(), token_mint.as_ref(), feed_account.as_ref(), price_update.as_ref()], program_id)
}

pub fn get_woopool_address(wooconfig: &Pubkey, token_mint: &Pubkey, quote_token_mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"woopool", wooconfig.as_ref(), token_mint.as_ref(), quote_token_mint.as_ref()], program_id)
}

pub fn get_wooammpool_address(wooconfig: &Pubkey, token_mint_a: &Pubkey, token_mint_b: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooammpool", wooconfig.as_ref(), token_mint_a.as_ref(), token_mint_b.as_ref()], program_id)
}

pub fn get_pubkey_from_param(param_map: &Map<String, Value>, key: String) -> Result<Pubkey> {
    let result = Pubkey::from_str(
        param_map.get(&key).context("missing key in param map")?
                 .as_str().context("param item is not correct")?
    )?;

    Ok(result)
}