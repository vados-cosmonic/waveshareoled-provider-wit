use async_trait::async_trait;

use wasmcloud_core::LinkDefinition;
use wasmcloud_provider_sdk::{Context, error::ProviderResult};

wasmcloud_provider_wit_bindgen::generate!(
    WaveshareOledProvider,
    "cosmonic:waveshareoled",
    "waveshareoled"
);

/// Implementation struct for Waveshare OLED provider
#[derive(Debug, Clone)]
pub struct WaveshareOledProvider;

#[async_trait]
impl WasmcloudCapabilityProvider for WaveshareOledProvider {
    async fn put_link(&self, ld: &LinkDefinition) -> bool {
        false // TODO
    }

    async fn delete_link(&self, actor_id: &str) {
        panic!("not implemented"); // TODO
    }

    async fn shutdown(&self) {
        panic!("not implemented") // TODO
    }
}

#[async_trait]
impl CosmonicWaveshareoledScreen for WaveshareOledProvider {
    async fn draw_message(&self, ctx: Context, msg: String) -> ProviderResult<Result<(), String>> {
        panic!("NOPE");
    }
}
