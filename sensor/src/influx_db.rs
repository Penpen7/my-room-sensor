use crate::notification::Notification;

pub struct InfluxDBNotification {
    influxdb: influxdb::Client,
}

impl InfluxDBNotification {
    pub async fn new(url: &str, bucket: &str, token: &str) -> InfluxDBNotification {
        // try to connect to InfluxDB 5 times with 5 seconds interval if it fails
        // when the connection is successful, it will return the client
        for _ in 0..5 {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let client = influxdb::Client::new(url, bucket).with_token(token);
            if client.ping().await.is_ok() {
                println!("Connected to InfluxDB");
                return InfluxDBNotification { influxdb: client };
            }
        }
        panic!("Failed to connect to InfluxDB");
    }
}

#[async_trait::async_trait]
impl Notification for InfluxDBNotification {
    async fn send_notification(
        &self,
        temperature: f32,
        humidity: f32,
        pressure: f32,
        gas_resistance: u32,
        co2_density: u16,
        discomfort_index: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let now = std::time::SystemTime::now();
        let timestamp = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let timestamp = influxdb::Timestamp::Nanoseconds(timestamp);
        let write_query = influxdb::WriteQuery::new(timestamp, "my_measurement")
            .add_tag("place", "Living Room")
            .add_field("Temperature", temperature)
            .add_field("Humidity", humidity)
            .add_field("Pressure", pressure)
            .add_field("Gas Resistance", gas_resistance)
            .add_field("CO2 Density", co2_density)
            .add_field("Discomfort Index", discomfort_index);

        let result = self.influxdb.query(&write_query).await?;
        if result != "" {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                result,
            )));
        }

        Ok(())
    }
}
