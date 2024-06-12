#[async_trait::async_trait]
pub trait Notification {
    async fn send_notification(
        &self,
        temperature: f32,
        humidity: f32,
        pressure: f32,
        gas_resistance: u32,
        co2_density: u16,
        discomfort_index: f32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
