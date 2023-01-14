mod simulator;

use std::time::Duration;

use crate::simulator::setup_sensor;
use ::vl53l3cx_driver::{DistanceMode, Roi};

test_single!(get_product_revision());

test_single!(get_device_info());

test_single!(i2c, delay, get_uid());

test_single!(i2c, set_device_address(0));

test_single!(set_distance_mode(DistanceMode::Short));

test_single!(get_distance_mode());

test_single!(set_measurement_timing_budget(Duration::from_millis(100)));

test_single!(get_measurement_timing_budget());

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

test_single!(set_roi(Roi {
    top_left_x: 0,
    top_left_y: 15,
    bottom_right_x: 15,
    bottom_right_y: 0
}));

test_single!(get_roi());