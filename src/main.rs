//! cosmonic:waveshareoled provider
//!
use anyhow::{Context, Result};
use wasmcloud_provider_sdk::start_provider;

use waveshareoled_provider_wit::WaveshareOledProvider;

fn main() -> Result<()> {
    let provider = WaveshareOledProvider;
    start_provider(provider, Some(String::from("Waveshare OLED Provider")))
        .context("failed to start provider")?;
    eprintln!("waveshare OLED provider exiting");
    Ok(())
}
