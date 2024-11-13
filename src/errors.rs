use std::num::TryFromIntError;

use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Unable to divide by zero")]
    DivideByZero, // 0x1770
    #[msg("Unable to cast number into BigInt")]
    NumberCastError, //  0x1771

    #[msg("Exceeded max fee rate")]
    FeeRateMaxExceeded, // 0x1772
    #[msg("Mathematical operation with overflow")]
    MathOverflow, // 0x1773
    #[msg("Muldiv overflow")]
    MulDivOverflow, // 0x1774
    #[msg("Exceeded max protocol fee")]
    ProtocolFeeMaxExceeded, // 0x1775
    #[msg("Protocol fee not enough")]
    ProtocolFeeNotEnough, // 0x1776
    #[msg("Exceeded max rebate fee")]
    RebateFeeMaxExceeded, // 0x1777
    #[msg("Rebate fee not enough")]
    RebateFeeNotEnough, // 0x1778
    #[msg("Exceeded max reserve")]
    ReserveMaxExceeded, // 0x1779
    #[msg("Reserve not enough")]
    ReserveNotEnough, // 0x177a
    #[msg("Reserve less than fee")]
    ReserveLessThanFee, // 0x177b

    #[msg("Too Many Authorities")]
    TooManyAuthorities, // 0x177c
    #[msg("Woo oracle bound exceed limit")]
    WooOracleBoundLimit, //0x177d

    #[msg("Woo oracle is not feasible")]
    WooOracleNotFeasible, //0x177e
    #[msg("Woo oracle price is not valid")]
    WooOraclePriceNotValid, //0x177f
    #[msg("Woo oracle price below range MIN")]
    WooOraclePriceRangeMin, //0x1780
    #[msg("Woo oracle price exceed range MAX")]
    WooOraclePriceRangeMax, //0x1781
    #[msg("Woo oracle spread exceed 1E18")]
    WooOracleSpreadExceed, //0x1782

    #[msg("Woo pp exceed max notional value")]
    WooPoolExceedMaxNotionalValue, //0x1783
    #[msg("Woo pp exceed max gamma")]
    WooPoolExceedMaxGamma, //0x1784

    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance, //0x1785
    #[msg("Not enough out")]
    NotEnoughOut, //0x1786
    #[msg("Amount out below minimum threshold")]
    AmountOutBelowMinimum, //0x1787
    #[msg("Amount exceeds max balance cap")]
    BalanceCapExceeds, //0x1788
    #[msg("Swap pool invalid")]
    SwapPoolInvalid, //0x1789

    #[msg("invalid authority")]
    InvalidAuthority,
}

impl From<TryFromIntError> for ErrorCode {
    fn from(_: TryFromIntError) -> Self {
        ErrorCode::NumberCastError
    }
}
