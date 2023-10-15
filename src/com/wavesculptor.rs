use crate::com::MessageFormat;
use crate::device::{source_address, Device};
use bxcan::{ExtendedId, Frame};
use j1939::pgn::{Number, Pgn};

use super::Priority;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DriverModes {
    Neutral = 1,
    Drive = 0,
    Reverse = 2,
}

impl From<u8> for DriverModes {
    fn from(val: u8) -> DriverModes {
        match val {
            1 => DriverModes::Neutral,
            0 => DriverModes::Drive,
            2 => DriverModes::Reverse,
            _ => DriverModes::Neutral // Default should be neutral
        }
    }
}

pub const PGN_SPEED_MESSAGE: Number = Number {
    specific: Device::SteeringWheel as u8,
    format: MessageFormat::Speed as u8,
    data_page: false,
    extended_data_page: false,
};

pub const PGN_BATTERY_MESSAGE: Number = Number {
    specific: Device::SteeringWheel as u8,
    format: MessageFormat::Battery as u8,
    data_page: false,
    extended_data_page: false,
};

pub const PGN_TEMPERATURE_MESSAGE: Number = Number {
    specific: Device::SteeringWheel as u8,
    format: MessageFormat::Temperature as u8,
    data_page: false,
    extended_data_page: false,
};

pub const PGN_SET_DRIVE_CONTROL_TYPE: Number = Number {
    specific: Device::VehicleController as u8,
    format: MessageFormat::ControlType as u8,
    data_page: false,
    extended_data_page: false
};

pub const PGN_SET_DRIVER_MODE: Number = Number {
    specific: Device::VehicleController as u8,
    format: MessageFormat::DriverMode as u8,
    data_page: false,
    extended_data_page: false
};

pub fn speed_message(device: Device, speed: f32) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_SPEED_MESSAGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), speed.to_le_bytes())
}

pub fn battery_message(device: Device, battery: f32) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_BATTERY_MESSAGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), battery.to_le_bytes())
}

pub fn temperature_message(device: Device, temp: f32) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_TEMPERATURE_MESSAGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), temp.to_le_bytes())
}

pub fn control_type_message(device: Device, control_type: bool) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_SET_DRIVE_CONTROL_TYPE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), [control_type as u8])
}

pub fn driver_mode_message(device: Device, driver_mode: DriverModes) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_SET_DRIVER_MODE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), [driver_mode as u8])
}