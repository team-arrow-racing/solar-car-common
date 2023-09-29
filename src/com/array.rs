//! Array controller messages

use crate::com::MessageFormat;
use crate::device::{source_address, Device};
use bxcan::{Data, ExtendedId, Frame};
use j1939::pgn::{Number, Pgn};

use super::Priority;

pub const PGN_START_PRECHARGE: Number = Number {
    specific: Device::ArrayIsolationController as u8,
    format: MessageFormat::Enable as u8,
    data_page: false,
    extended_data_page: false,
};

pub fn start_precharge(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Control as u8,
        pgn: Pgn::new(PGN_START_PRECHARGE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}

pub const PGN_ISOLATE: Number = Number {
    specific: Device::ArrayIsolationController as u8,
    format: MessageFormat::Disable as u8,
    data_page: false,
    extended_data_page: false,
};

pub fn isolate(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Important as u8,
        pgn: Pgn::new(PGN_ISOLATE),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}

pub const PGN_FEED_WATCHDOG: Number = Number {
    specific: Device::ArrayIsolationController as u8,
    format: MessageFormat::Heartbeat as u8,
    data_page: false,
    extended_data_page: false,
};

pub fn feed_watchdog(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Control as u8,
        pgn: Pgn::new(PGN_FEED_WATCHDOG),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}

pub const PGN_ENABLE_CONTACTORS: Number = Number {
    specific: Device::VehicleController as u8,
    format: MessageFormat::Enable as u8,
    data_page: false,
    extended_data_page: false,
};

pub fn enable_contactors_message(device: Device, state: bool) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Critical as u8,
        pgn: Pgn::new(PGN_ENABLE_CONTACTORS),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), [state as u8])
}
