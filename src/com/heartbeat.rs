///! Standardised heartbeat messaging.

use sae_j1939::IdExtended;
use bxcan::{Data, Frame, ExtendedId};
use crate::device::{Device, source_address};
use crate::com::Priority;
use crate::com::{MessageFormat, GroupExtension};

/// Construct a message to be sent at regular intervals with status information.
pub fn message(device: Device) -> Frame {
    let id = IdExtended {
        priority: Priority::Default as u8,
        ext_data_page: false,
        data_page: false,
        pdu_format: MessageFormat::Heartbeat as u8,
        pdu_specific: GroupExtension::Default as u8,
        source_address: source_address(device).unwrap(),
    };

    // TODO: decicde on heartbeat contents
    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}
