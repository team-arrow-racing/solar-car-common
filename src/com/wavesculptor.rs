use crate::com::MessageFormat;
use crate::device::{source_address, Device};
use bxcan::{ExtendedId, Frame};
use j1939::pgn::{Number, Pgn};

use super::Priority;

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

pub fn speed_message(device: Device, speed: f32) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_SPEED_MESSAGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), [speed as u8])
}

pub fn battery_message(device: Device, battery: f32) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_BATTERY_MESSAGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), [battery as u8])
}

pub fn temperature_message(device: Device, temp: f32) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_TEMPERATURE_MESSAGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), [temp as u8])
}