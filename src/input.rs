use std::fs;

use crate::prelude::*;

pub fn load_day(day: usize) -> Result<String> {
    let file_name = format!("{day:02}");
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}
