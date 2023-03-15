use crate::com::Priority;
use crate::com::{GroupExtension, MessageFormat};
use crate::device::{source_address, Device};
///! Standardised heartbeat messaging.
use bxcan::{Data, ExtendedId, Frame};
use j1939::pgn::{Number, Pgn};

const PGN_HEARTBEAT: Number = Number {
    specific: GroupExtension::Default as u8,
    format: MessageFormat::Heartbeat as u8,
    data_page: false,
    extended_data_page: false,
};

/// Construct a message to be sent at regular intervals with status information.
pub fn message(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Default as u8,
        pgn: Pgn::new(PGN_HEARTBEAT),
        source_address: source_address(device).unwrap(),
    };

    // TODO: decicde on heartbeat contents
    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}
