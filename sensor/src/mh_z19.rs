use crate::sensors::CO2DensitySensor;
use serialport::{self, DataBits, Parity, StopBits};
use std::io::{self, Read, Write};
use std::time::Duration;

const SERIAL_TIMEOUT: Duration = Duration::from_millis(2000);
const S_DATA: [u8; 9] = [0xFF, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79];

pub struct Mhz19 {
    port: Box<dyn serialport::SerialPort>,
}

impl Mhz19 {
    pub fn new(port_path: &str) -> Result<Mhz19, Box<dyn std::error::Error>> {
        let port = serialport::new(port_path, 9600)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(SERIAL_TIMEOUT)
            .open()?;

        Ok(Mhz19 { port })
    }

    fn calculate_checksum(&self, data: &[u8]) -> u8 {
        255 - (data[1] + data[2] + data[3] + data[4] + data[5] + data[6] + data[7]) + 1
    }
}

impl CO2DensitySensor for Mhz19 {
    fn read_co2_density(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        self.port.write_all(&S_DATA)?;

        let mut rdata: [u8; 9] = [0; 9];
        self.port.read_exact(&mut rdata)?;

        let checksum = self.calculate_checksum(&rdata);
        if rdata[8] != checksum {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Bad Checksum: Checksum value: {} / Calculated value: {}",
                    rdata[8], checksum
                ),
            )
            .into());
        }

        Ok(rdata[2] as u16 * 256 + rdata[3] as u16)
    }
}
