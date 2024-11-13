use crate::errors::ErrorCode;

#[derive(Clone, Default, Copy)]
pub struct Decimals {
    pub price_dec: u64, // 10 ** 8
    pub quote_dec: u64, // 10 ** 6, same as native USDC
    pub base_dec: u64,  // 10 ** 18 or 8
}

impl Decimals {
    pub fn new(price: u32, quote: u32, base: u32) -> Result<Decimals, ErrorCode> {
        Ok(Decimals {
            price_dec: 10_u64.checked_pow(price).ok_or(ErrorCode::MathOverflow)?,
            quote_dec: 10_u64.checked_pow(quote).ok_or(ErrorCode::MathOverflow)?,
            base_dec: 10_u64.checked_pow(base).ok_or(ErrorCode::MathOverflow)?,
        })
    }
}
