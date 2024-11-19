#[derive(Clone, Default, Copy)]
pub struct Decimals {
    pub price_dec: u64, // 10 ** 8
    pub quote_dec: u64, // 10 ** 6, same as native USDC
    pub base_dec: u64,  // 10 ** 18 or 8
}

impl Decimals {
    pub fn new(price: u32, quote: u32, base: u32) -> Option<Decimals> {
        let (
                Some(price_dec), 
                Some(quote_dec),
                Some(base_dec)
            ) = 
            (
                10_u64.checked_pow(price),
                10_u64.checked_pow(quote),
                10_u64.checked_pow(base)
            )
            else {
                return None
            };
        
        Some(Decimals {
            price_dec,
            quote_dec,
            base_dec,
        })
    }
}
