use my_room_sensor::bme680::BME680;
use my_room_sensor::influx_db::InfluxDBNotification;
use my_room_sensor::mh_z19::Mhz19;
use my_room_sensor::run;
use my_room_sensor::sensors::{MockSensor, Sensors};
use std::cell::RefCell;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let env = std::env::var("ENV").unwrap();
    let url = std::env::var("INFLUXDB_URL").unwrap();
    let bucket = std::env::var("INFLUXDB_BUCKET").unwrap();
    let token = std::env::var("INFLUXDB_TOKEN").unwrap();

    let notification = InfluxDBNotification::new(&url, &bucket, &token).await;
    match env.as_str() {
        "local" => {
            let mock_sensor = Rc::new(RefCell::new(MockSensor::new()));
            let sensor = Sensors::new(
                mock_sensor.clone(),
                mock_sensor.clone(),
                mock_sensor.clone(),
                mock_sensor.clone(),
                mock_sensor,
            )
            .unwrap();
            run(sensor, vec![Box::new(notification)]).await;
        }
        _ => {
            let mhz19 = Rc::new(RefCell::new(Mhz19::new("/dev/ttyS0").unwrap()));
            let bme680 = BME680::new("/dev/i2c-1").unwrap();
            let bme680 = Rc::new(RefCell::new(bme680));
            let sensor = Sensors::new(
                bme680.clone(),
                bme680.clone(),
                bme680.clone(),
                bme680,
                mhz19,
            )
            .unwrap();
            run(sensor, vec![Box::new(notification)]).await;
        }
    };
}
