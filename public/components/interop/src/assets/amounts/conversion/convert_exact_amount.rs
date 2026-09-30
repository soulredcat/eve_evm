use crate::{InteropError, U256};

/// Convert positive native base units without rounding or narrowing.
pub fn convert_exact_amount(
    amount: U256,
    source_decimals: u8,
    destination_decimals: u8,
    destination_maximum: U256,
) -> Result<U256, InteropError> {
    if source_decimals > 77 || destination_decimals > 77 {
        return Err(InteropError::InvalidDecimals);
    }
    if amount == U256::ZERO {
        return Err(InteropError::ZeroAmount);
    }
    let difference = source_decimals.abs_diff(destination_decimals);
    let mut factor = U256::from(1);
    for _ in 0..difference {
        factor = factor
            .checked_mul(U256::from(10))
            .ok_or(InteropError::AmountOverflow)?;
    }
    let converted = if source_decimals > destination_decimals {
        if amount % factor != U256::ZERO {
            return Err(InteropError::NonExactAmount);
        }
        amount / factor
    } else {
        amount
            .checked_mul(factor)
            .ok_or(InteropError::AmountOverflow)?
    };
    if converted > destination_maximum {
        return Err(InteropError::AmountOverflow);
    }
    Ok(converted)
}
