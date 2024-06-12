use std::cell::RefCell;
use std::rc::Rc;

pub trait TemperatureSensor {
    fn read_temperature(&mut self) -> Result<f32, Box<dyn std::error::Error>>;
}

pub trait HumiditySensor {
    fn read_humidity(&mut self) -> Result<f32, Box<dyn std::error::Error>>;
}

pub trait PressureSensor {
    fn read_pressure(&mut self) -> Result<f32, Box<dyn std::error::Error>>;
}

pub trait GasResistanceSensor {
    fn read_gas_resistance(&mut self) -> Result<u32, Box<dyn std::error::Error>>;
}

pub trait CO2DensitySensor {
    fn read_co2_density(&mut self) -> Result<u16, Box<dyn std::error::Error>>;
}

pub struct Sensors<T, H, P, G, C> {
    temperature_sensor: Rc<RefCell<T>>,
    humidity_sensor: Rc<RefCell<H>>,
    pressure_sensor: Rc<RefCell<P>>,
    gas_resistance_sensor: Rc<RefCell<G>>,
    co2_density_sensor: Rc<RefCell<C>>,
}

impl<T, H, P, G, C> Sensors<T, H, P, G, C>
where
    T: TemperatureSensor,
    H: HumiditySensor,
    P: PressureSensor,
    G: GasResistanceSensor,
    C: CO2DensitySensor,
{
    pub fn new(
        temperature_sensor: Rc<RefCell<T>>,
        humidity_sensor: Rc<RefCell<H>>,
        pressure_sensor: Rc<RefCell<P>>,
        gas_resistance_sensor: Rc<RefCell<G>>,
        co2_density_sensor: Rc<RefCell<C>>,
    ) -> Result<Sensors<T, H, P, G, C>, Box<dyn std::error::Error>> {
        Ok(Sensors {
            temperature_sensor,
            humidity_sensor,
            pressure_sensor,
            gas_resistance_sensor,
            co2_density_sensor,
        })
    }

    pub fn read_temperature(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        self.temperature_sensor.borrow_mut().read_temperature()
    }

    pub fn read_humidity(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        self.humidity_sensor.borrow_mut().read_humidity()
    }

    pub fn read_pressure(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        self.pressure_sensor.borrow_mut().read_pressure()
    }

    pub fn read_gas_resistance(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        self.gas_resistance_sensor
            .borrow_mut()
            .read_gas_resistance()
    }

    pub fn read_co2_density(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        self.co2_density_sensor.borrow_mut().read_co2_density()
    }
}

pub struct MockSensor {}

impl MockSensor {
    pub fn new() -> MockSensor {
        MockSensor {}
    }
}

impl TemperatureSensor for MockSensor {
    fn read_temperature(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(25.0)
    }
}

impl HumiditySensor for MockSensor {
    fn read_humidity(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(50.0)
    }
}

impl PressureSensor for MockSensor {
    fn read_pressure(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(1000.0)
    }
}

impl GasResistanceSensor for MockSensor {
    fn read_gas_resistance(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        Ok(10000)
    }
}

impl CO2DensitySensor for MockSensor {
    fn read_co2_density(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        Ok(400)
    }
}
