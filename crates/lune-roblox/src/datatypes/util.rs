/**
    Converts an optional Luau number into a 16-bit integer component,
    the way Roblox constructors do it: a fraction is truncated,
    and a value outside of the 16-bit range is an error.
*/
pub fn int16_from(value: Option<f64>) -> mlua::Result<i32> {
    let value = value.unwrap_or_default().trunc();
    if value.is_finite() && value >= f64::from(i16::MIN) && value <= f64::from(i16::MAX) {
        Ok(value as i32)
    } else {
        Err(mlua::Error::RuntimeError(format!(
            "number {value} is out of range for a 16-bit integer"
        )))
    }
}
