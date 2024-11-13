use solana_program::{
    pubkey,
    pubkey::Pubkey,
    system_program,
};

pub const WOOFI_PROGRAM_ID: Pubkey = pubkey!("woocYbcZkJ1ryopvtNP7Lr367wbW4WrMgxmzroe6VWU");

pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub fn get_vault_address(market: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault", market.as_ref(), mint.as_ref()], &WOOFI_PROGRAM_ID)
}

pub fn get_seat_address(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"seat", market.as_ref(), trader.as_ref()], &WOOFI_PROGRAM_ID)
}