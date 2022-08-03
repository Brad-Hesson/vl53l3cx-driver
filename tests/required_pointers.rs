use ::vl53l3cx_driver::DistanceMode;
use sim::setup_sensor;

test_single!(get_product_revision());
test_single!(get_device_info());
test_single!(i2c, delay, get_uid());
test_single!(i2c, set_device_address(0));
test_single!(set_distance_mode(DistanceMode::Short));
test_single!(get_distance_mode());
test_single!(set_measurement_timing_budget_ms(100));
test_single!(get_measurement_timing_budget_ms());
test_single!(i2c, start_measurement());
test_single!(i2c, stop_measurement());
test_single!(i2c, clear_interrupt_and_start_measurement());
test_single!(i2c, get_measurement_data_ready());
#[test]
fn wait_measurement_data_ready() {
    let (mut sensor, mut i2c, mut delay) = setup_sensor();
    sensor.start_measurement(&mut i2c).unwrap();
    assert!(i2c.used());
    sensor
        .wait_measurement_data_ready(&mut i2c, &mut delay)
        .unwrap();
    assert!(i2c.used());
    assert!(delay.used());
}
test_single!(i2c, get_multiranging_data());
test_single!(get_additional_data());

#[test]
fn variance() {
    let (mut sensor, mut i2c, mut delay) = setup_sensor();
    sensor.start_measurement(&mut i2c).unwrap();
    sensor
        .wait_measurement_data_ready(&mut i2c, &mut delay)
        .unwrap();
    sensor = sensor
        .into_disabled()
        .into_enabled(&mut i2c, &mut delay)
        .unwrap();
    sensor.start_measurement(&mut i2c).unwrap();
    sensor
        .wait_measurement_data_ready(&mut i2c, &mut delay)
        .unwrap();
}

mod sim {
    extern crate std;
    use ::embedded_hal::{
        blocking::{
            delay::{DelayMs, DelayUs},
            i2c::{Read, Write},
        },
        digital::v2::OutputPin,
    };
    use ::std::{eprint, eprintln};
    use ::vl53l3cx_driver::{Enabled, VL53L3CX};
    use std::{cell::RefCell, rc::Rc};

    pub fn setup_sensor<'a>() -> (VL53L3CX<'a, Enabled, I2c, Pin, Delay>, I2c, Delay) {
        let device = Rc::new(RefCell::new(DeviceSim::new()));
        let mut i2c = I2c::new(device.clone());
        let pin = Pin(device);
        let mut delay = Delay::new();
        let sensor = VL53L3CX::new(0x00, pin)
            .into_enabled(&mut i2c, &mut delay)
            .unwrap();
        assert!(i2c.used());
        assert!(delay.used());
        eprintln!("-------- sensor setup complete --------");
        (sensor, i2c, delay)
    }

    #[macro_export]
    macro_rules! test_single {
        (i2c, delay, $fn:tt($($args:expr),*)) => {
            #[test]
            fn $fn() {
                let (mut sensor, mut i2c, mut delay) = setup_sensor();
                let result = sensor.$fn(&mut i2c, &mut delay, $($args),*).unwrap();
                eprintln!("{:?}", result);
                assert!(i2c.used(), "Expected use of i2c, but it was not used");
                assert!(delay.used(), "Expected use of delay, but it was not used");
            }
        };
        (i2c, $fn:tt($($args:expr),*)) => {
            #[test]
            fn $fn() {
                let (mut sensor, mut i2c, _) = setup_sensor();
                let result = sensor.$fn(&mut i2c, $($args),*).unwrap();
                eprintln!("{:?}", result);
                assert!(i2c.used(), "Expected use of i2c, but it was not used");
            }
        };
        ($fn:tt($($args:expr),*)) => {
            #[test]
            fn $fn() {
                let (mut sensor, _, _) = setup_sensor();
                let result = sensor.$fn($($args),*).unwrap();
                eprintln!("{:?}", result);
            }
        };
    }

    pub struct DeviceSim {
        state: DeviceState,
        memory: [u8; 0xFFFF],
        read_index: Option<u16>,
    }
    impl DeviceSim {
        pub fn new() -> Self {
            let mut _self = Self {
                state: DeviceState::Off,
                read_index: None,
                memory: [0xFF; 0xFFFF],
            };
            _self.write_memory(0x00E5, &[0]);
            _self
        }
        fn update(&mut self) {
            match self.state {
                DeviceState::Off => {}
                DeviceState::Booting(0) => {
                    self.state = DeviceState::Idle;
                    self.write_memory(0x00E5, &[0x03]);
                }
                DeviceState::Booting(n) => self.state = DeviceState::Booting(n - 1),
                DeviceState::Idle => {
                    let mut buf = [0u8];
                    self.read_memory(0x31, &mut buf);
                    if buf[0] == 0x02 {
                        self.write_memory(0x0031, &[0x03]);
                        self.state = DeviceState::Measurement(10);
                    }
                }
                DeviceState::Measurement(0) => {
                    self.state = DeviceState::Idle;
                    self.write_memory(0x31, &[0x02]);
                }
                DeviceState::Measurement(n) => self.state = DeviceState::Measurement(n - 1),
            }
        }
        fn write_memory(&mut self, index: u16, data: &[u8]) {
            let start = index as usize;
            let end = start + data.len();
            self.memory[start..end].copy_from_slice(data);
        }
        fn read_memory(&self, index: u16, buffer: &mut [u8]) {
            let start = index as usize;
            let end = start + buffer.len();
            buffer.copy_from_slice(&self.memory[start..end]);
        }
    }
    enum DeviceState {
        Off,
        Booting(u8),
        Idle,
        Measurement(u8),
    }

    pub struct Pin(Rc<RefCell<DeviceSim>>);
    impl OutputPin for Pin {
        type Error = ();

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.0.borrow_mut().state = DeviceState::Off;
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.0.borrow_mut().state = DeviceState::Booting(10);
            Ok(())
        }
    }

    pub struct I2c {
        device: Rc<RefCell<DeviceSim>>,
        used: bool,
    }
    impl I2c {
        pub fn new(device: Rc<RefCell<DeviceSim>>) -> Self {
            Self {
                device,
                used: false,
            }
        }
        pub fn used(&mut self) -> bool {
            let result = self.used == true;
            self.reset();
            result
        }
        fn reset(&mut self) {
            self.used = false;
        }
    }
    impl Read for I2c {
        type Error = ();

        fn read(&mut self, _address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            let index = self
                .device
                .borrow_mut()
                .read_index
                .take()
                .expect("read address was not set before read");
            self.device.borrow().read_memory(index, buffer);
            eprint!(
                " Read: [0x{:04X}..0x{:04X}] => ",
                index,
                index + buffer.len() as u16 - 1
            );
            for byte in buffer.iter() {
                eprint!("0x{:02X} ", byte);
            }
            eprintln!();
            Ok(())
        }
    }
    impl Write for I2c {
        type Error = ();

        fn write(&mut self, _address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.device.borrow_mut().update();
            self.used = true;
            match bytes.len() {
                2 => {
                    self.device.borrow_mut().read_index = Some(index_from_bytes(bytes));
                }
                n if n > 2 => {
                    let index = index_from_bytes(bytes);
                    eprint!(
                        "Write: [0x{:04X}..0x{:04X}] <= ",
                        index,
                        bytes.len() as u16 - 2 + index - 1
                    );
                    for byte in &bytes[2..] {
                        eprint!("0x{:02X} ", byte);
                    }
                    eprintln!();
                    self.device.borrow_mut().write_memory(index, &bytes[2..]);
                }
                _ => unreachable!(),
            }
            Ok(())
        }
    }
    fn index_from_bytes(bytes: &[u8]) -> u16 {
        (bytes[0] as u16) << 8 | bytes[1] as u16
    }
    pub struct Delay {
        used: bool,
    }
    impl Delay {
        pub fn new() -> Self {
            Self { used: false }
        }
        pub fn used(&mut self) -> bool {
            let result = self.used == true;
            self.reset();
            result
        }
        fn reset(&mut self) {
            self.used = false;
        }
    }
    impl DelayMs<u32> for Delay {
        fn delay_ms(&mut self, _ms: u32) {
            self.used = true;
        }
    }
    impl DelayUs<u32> for Delay {
        fn delay_us(&mut self, _us: u32) {
            self.used = true;
        }
    }
}
