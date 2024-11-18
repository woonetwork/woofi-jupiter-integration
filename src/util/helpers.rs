use anchor_lang::accounts::program;
use solana_program::{
    pubkey,
    pubkey::Pubkey,
    system_program,
};

// PROD
// pub const WOOFI_PROGRAM_ID: Pubkey = pubkey!("woocYbcZkJ1ryopvtNP7Lr367wbW4WrMgxmzroe6VWU");

// pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
// pub const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

// DEV
// pub const WOOFI_PROGRAM_ID: Pubkey = pubkey!("Es677W33uwrXLSqjV3rqcz5sftyarupdV3vDpQ9LXGow");

pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

pub const SOL_FEED_ACCOUNT: Pubkey = pubkey!("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG");
pub const USDC_FEED_ACCOUNT: Pubkey = pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");

pub const SOL_PRICE_UPDATE: Pubkey = pubkey!("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");
pub const USDC_PRICE_UPDATE: Pubkey = pubkey!("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX");

pub fn get_wooconfig_address(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooconfig"], program_id)
}

pub fn get_wooracle_address(wooconfig: &Pubkey, token_mint: &Pubkey, feed_account: &Pubkey, price_update: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooracle", wooconfig.as_ref(), token_mint.as_ref(), feed_account.as_ref(), price_update.as_ref()], program_id)
}

pub fn get_woopool_address(wooconfig: &Pubkey, token_mint: &Pubkey, quote_token_mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"woopool", wooconfig.as_ref(), token_mint.as_ref(), quote_token_mint.as_ref()], program_id)
}