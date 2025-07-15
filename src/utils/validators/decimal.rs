use bigdecimal::BigDecimal;
use validator::ValidationError;

pub fn validate_positive_decimal(value: &BigDecimal) -> Result<(), ValidationError> {
    if value <= &BigDecimal::from(0) {
        return Err(ValidationError::new("must_be_positive"));
    }
    Ok(())
}