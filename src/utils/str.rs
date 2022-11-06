use std::env::VarError;

pub fn stror(v1: String, v2: String) -> Result<String, VarError> {
    return Ok(if v1 != "" { v1 } else { v2 });
}