//! Array controller messages

use crate::com::{MessageFormat};
use crate::device::{source_address, Device};
use bxcan::{Data, ExtendedId, Frame};

use super::Priority;

pub fn start_precharge(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Control as u8,
        ext_data_page: false,
        data_page: false,
        pdu_format: MessageFormat::Enable as u8,
        pdu_specific: Device::ArrayIsolationController as u8,
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}

pub fn isolate(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: Priority::Important as u8,
        ext_data_page: false,
        data_page: false,
        pdu_format: MessageFormat::Disable as u8,
        pdu_specific: Device::ArrayIsolationController as u8,
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}
