use enigo::{Enigo, Keyboard};
use std::{thread, time::Duration};
use anyhow::{Result, Context};

use crate::config::CONFIG;

pub fn type_text(enigo: &mut Enigo, word: &str) -> Result<()> {

    for ch in word.split("") {
        enigo.text(ch).with_context(|| format!("Fail when typing char {}", ch))?;
        let duration = fastrand::u64(CONFIG.delay_type_min..=CONFIG.delay_type_max);
        thread::sleep(Duration::from_millis(duration));
    }

    Ok(())
}