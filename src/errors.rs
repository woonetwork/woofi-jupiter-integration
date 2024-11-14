use std::num::TryFromIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCode {
    #[error("Unable to divide by zero")]
    DivideByZero, // 0x1770
    #[error("Unable to cast number into BigInt")]
    NumberCastError, //  0x1771

    #[error("Exceeded max fee rate")]
    FeeRateMaxExceeded, // 0x1772
    #[error("Mathematical operation with overflow")]
    MathOverflow, // 0x1773
    #[error("Muldiv overflow")]
    MulDivOverflow, // 0x1774
    #[error("Exceeded max protocol fee")]
    ProtocolFeeMaxExceeded, // 0x1775
    #[error("Protocol fee not enough")]
    ProtocolFeeNotEnough, // 0x1776
    #[error("Exceeded max rebate fee")]
    RebateFeeMaxExceeded, // 0x1777
    #[error("Rebate fee not enough")]
    RebateFeeNotEnough, // 0x1778
    #[error("Exceeded max reserve")]
    ReserveMaxExceeded, // 0x1779
    #[error("Reserve not enough")]
    ReserveNotEnough, // 0x177a
    #[error("Reserve less than fee")]
    ReserveLessThanFee, // 0x177b

    #[error("Too Many Authorities")]
    TooManyAuthorities, // 0x177c
    #[error("Woo oracle bound exceed limit")]
    WooOracleBoundLimit, //0x177d

    #[error("Woo oracle is not feasible")]
    WooOracleNotFeasible, //0x177e
    #[error("Woo oracle price is not valid")]
    WooOraclePriceNotValid, //0x177f
    #[error("Woo oracle price below range MIN")]
    WooOraclePriceRangeMin, //0x1780
    #[error("Woo oracle price exceed range MAX")]
    WooOraclePriceRangeMax, //0x1781
    #[error("Woo oracle spread exceed 1E18")]
    WooOracleSpreadExceed, //0x1782

    #[error("Woo pp exceed max notional value")]
    WooPoolExceedMaxNotionalValue, //0x1783
    #[error("Woo pp exceed max gamma")]
    WooPoolExceedMaxGamma, //0x1784

    #[error("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance, //0x1785
    #[error("Not enough out")]
    NotEnoughOut, //0x1786
    #[error("Amount out below minimum threshold")]
    AmountOutBelowMinimum, //0x1787
    #[error("Amount exceeds max balance cap")]
    BalanceCapExceeds, //0x1788
    #[error("Swap pool invalid")]
    SwapPoolInvalid, //0x1789

    #[error("invalid authority")]
    InvalidAuthority,
}

impl From<TryFromIntError> for ErrorCode {
    fn from(_: TryFromIntError) -> Self {
        ErrorCode::NumberCastError
    }
}
