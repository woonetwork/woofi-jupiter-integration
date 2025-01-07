pub const WOORACLE_SEED: &str = "wooracle";
pub const WOOPOOL_SEED: &str = "woopool";
pub const WOOCONFIG_SEED: &str = "wooconfig";
pub const WOOAMMPOOL_SEED: &str = "wooammpool";

pub const ONE_E5_U128: u128 = 100_000;
pub const ONE_E18_U64: u64 = 1_000_000_000_000_000_000;
pub const ONE_E18_U128: u128 = 1_000_000_000_000_000_000;
pub const DEFAULT_BOUND: u64 = 10_000_000_000_000_000; // 1e16 means 1%
                                                       // unix timestamp in seconds
pub const DEFAULT_STALE_DURATION: i64 = 120; // Default to 120s

// unit: 0.1 bps (1e5 = 100%, 25 = 2.5 bps)
// Fee amount = total_amount * fee_rate / 100_000.
// Max fee rate supported is u16::MAX around 65.5%.
pub const MAX_FEE_RATE: u16 = u16::MAX - 1;
