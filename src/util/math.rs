use crate::errors::ErrorCode;

pub fn checked_mul_div(n0: u128, n1: u128, d: u128) -> Result<u128, ErrorCode> {
    checked_mul_div_round_up_if(n0, n1, d, false)
}

pub fn checked_mul_div_round_up(n0: u128, n1: u128, d: u128) -> Result<u128, ErrorCode> {
    checked_mul_div_round_up_if(n0, n1, d, true)
}

pub fn checked_mul_div_round_up_if(
    n0: u128,
    n1: u128,
    d: u128,
    round_up: bool,
) -> Result<u128, ErrorCode> {
    if d == 0 {
        return Err(ErrorCode::DivideByZero);
    }

    let p = n0.checked_mul(n1).ok_or(ErrorCode::MulDivOverflow)?;
    let n = p / d;

    Ok(if round_up && p % d > 0 { n + 1 } else { n })
}
