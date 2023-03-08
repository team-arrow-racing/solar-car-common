///! Standardised heartbeat messaging.

use sae_j1939::IdExtended;
use bxcan::{Data, Frame, ExtendedId};
use crate::device::{Device, source_address};
use crate::com::Priority;

/// Construct a message to be sent at regular intervals with status information.
pub fn message(device: Device) -> Frame {
    let id = IdExtended {
        priority: Priority::Default as u8,
        ext_data_page: false,
        data_page: false,
        pdu_format: 0xFF,
        pdu_specific: 0x00,
        source_address: source_address(device).unwrap(),
    };

    // TODO: decicde on heartbeat contents
    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}
