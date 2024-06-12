use crate::cache::TTLCache;
use crate::sensors::{GasResistanceSensor, HumiditySensor, PressureSensor, TemperatureSensor};
use bme680::*;
use chrono::TimeDelta;
use core::result;
use core::time::Duration;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{self as hal};
use log::info;

pub struct BME680 {
    dev: Bme680<hal::I2cdev, Delay>,
    cached_data: TTLCache<FieldData>,
}

impl BME680 {
    pub fn new(path: &str) -> result::Result<Self, Box<dyn std::error::Error>> {
        env_logger::init();
        let i2c = hal::I2cdev::new(path).unwrap();

        let mut delay = Delay {};
        let mut dev = Bme680::init(i2c, &mut delay, I2CAddress::Secondary).unwrap();

        let settings = SettingsBuilder::new()
            .with_humidity_oversampling(OversamplingSetting::OS2x)
            .with_pressure_oversampling(OversamplingSetting::OS4x)
            .with_temperature_oversampling(OversamplingSetting::OS8x)
            .with_temperature_filter(IIRFilterSize::Size3)
            .with_gas_measurement(Duration::from_millis(1500), 320, 25)
            .with_run_gas(true)
            .build();

        let profile_dur = dev.get_profile_dur(&settings.0).unwrap();
        info!("Profile duration {:?}", profile_dur);
        info!("Setting sensor settings");
        dev.set_sensor_settings(&mut delay, settings).unwrap();
        info!("Setting forced power modes");
        dev.set_sensor_mode(&mut delay, PowerMode::ForcedMode)
            .unwrap();
        let sensor_settings = dev.get_sensor_settings(settings.1);
        info!("Sensor settings: {:?}", sensor_settings);

        Ok(Self {
            dev,
            cached_data: TTLCache::new(TimeDelta::seconds(5)),
        })
    }

    pub fn fetch_data(&mut self) -> result::Result<FieldData, Box<dyn std::error::Error>> {
        let data = Box::new(|| {
            let mut delayer = Delay {};
            self.dev
                .set_sensor_mode(&mut delayer, PowerMode::ForcedMode)
                .unwrap();
            let (data, _state) = self.dev.get_sensor_data(&mut delayer).unwrap();
            data
        });
        Ok(*self.cached_data.get(data))
    }
}

impl TemperatureSensor for BME680 {
    fn read_temperature(&mut self) -> result::Result<f32, Box<dyn std::error::Error>> {
        let data = self.fetch_data()?;
        Ok(data.temperature_celsius())
    }
}

impl HumiditySensor for BME680 {
    fn read_humidity(&mut self) -> result::Result<f32, Box<dyn std::error::Error>> {
        let data = self.fetch_data()?;
        Ok(data.humidity_percent())
    }
}

impl PressureSensor for BME680 {
    fn read_pressure(&mut self) -> result::Result<f32, Box<dyn std::error::Error>> {
        let data = self.fetch_data()?;
        Ok(data.pressure_hpa())
    }
}

impl GasResistanceSensor for BME680 {
    fn read_gas_resistance(&mut self) -> result::Result<u32, Box<dyn std::error::Error>> {
        let data = self.fetch_data()?;
        Ok(data.gas_resistance_ohm())
    }
}
