pub mod bme680;
mod cache;
pub mod influx_db;
pub mod mh_z19;
pub mod notification;
pub mod sensors;

use crate::notification::Notification;
use crate::sensors::Sensors;

pub async fn run<T, H, P, G, C>(
    mut sensors: Sensors<T, H, P, G, C>,
    notification: Vec<Box<dyn Notification>>,
) where
    T: sensors::TemperatureSensor,
    H: sensors::HumiditySensor,
    P: sensors::PressureSensor,
    G: sensors::GasResistanceSensor,
    C: sensors::CO2DensitySensor,
{
    loop {
        let temperature = sensors.read_temperature().unwrap();
        let humidity = sensors.read_humidity().unwrap();
        let pressure = sensors.read_pressure().unwrap();
        let gas_resistance = sensors.read_gas_resistance().unwrap();
        let co2_density = sensors.read_co2_density().unwrap();
        let di = 0.81 * temperature + 0.01 * humidity * (0.99 * temperature - 14.3) + 46.3;

        println!(
            "Temperature: {}°C, Humidity: {}%, Pressure: {}hPa, Gas Resistance: {}Ω, CO2 Density: {}ppm",
            temperature, humidity, pressure, gas_resistance, co2_density
        );
        for n in notification.iter() {
            n.send_notification(
                temperature,
                humidity,
                pressure,
                gas_resistance,
                co2_density,
                di,
            )
            .await
            .unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
