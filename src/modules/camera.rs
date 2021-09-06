use crate::modules::Module;
use async_trait::async_trait;
use big_s::S;
pub use blackmagic_camera_control::BluetoothCamera;

#[async_trait]
impl Module for BluetoothCamera {
    fn name(&self) -> String {
        return S("camera");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        None
    }
}
