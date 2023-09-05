//! Keyvalue implementation for wasmcloud:keyvalue.
//!
use anyhow::Result;
use wasmcloud_provider_sdk::start_provider;

use waveshareoled_provider_wit::WaveshareOledProvider;

fn main() -> Result<()> {
    let provider = WaveshareOledProvider;
    start_provider(provider, Some("Waveshare OLED Provider".to_string()))?;
    eprintln!("waveshare OLED provider exiting");
    Ok(())
}
