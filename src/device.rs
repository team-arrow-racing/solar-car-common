use sae_j1939::IdExtended;
use bxcan::{Data, ExtendedId, Frame};
use bitflags::bitflags;
use stm32l4xx_hal::pac;

/// Vehicle devices
#[derive(Debug, Clone, Copy)]
pub enum Device {
    VehicleController,
    SteeringWheel,
    ArrayIsolationController,
    WaveSculptor,
    MpptA,
    MpptB,
}

/// Get J1939 source address
pub fn source_address(device: Device) -> Option<u8> {
    match device {
        Device::VehicleController => Some(0x10),
        Device::SteeringWheel => Some(0x20),
        Device::ArrayIsolationController => Some(0x30),
        _ => None
    }
}

/// Get base address for device (for legacy devices)
pub fn base_address(device: Device) -> Option<u16> {
    match device {
        Device::WaveSculptor => Some(0x400),
        Device::MpptA => Some(0x600),
        Device::MpptB => Some(0x610),
        _ => None,
    }
}

/// Construct a message to be sent at regular intervals with status information.
pub fn heartbeat_msg(device: Device) -> Frame {
    let id = IdExtended {
        priority: 6,
        ext_data_page: false,
        data_page: false,
        pdu_format: 0xFF,
        pdu_specific: 0x00,
        source_address: source_address(device).unwrap(),
    };

    // TODO: decicde on heartbeat contents
    Frame::new_data(ExtendedId::new(id.to_bits()).unwrap(), Data::empty())
}

/// To be sent on device initialisation.
pub fn startup_msg(device: Device) -> Frame {
    let id = IdExtended {
        priority: 3,
        ext_data_page: false,
        data_page: false,
        pdu_format: 0xFF,
        pdu_specific: 0x00,
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(
        ExtendedId::new(id.to_bits()).unwrap(),
        [reset_flags().bits()],
    )
}

bitflags! {
    /// Last reset cause
    struct LastResetCause: u8 {
        const HARDWARE = (1 << 0);
        const SOFTWARE = (1 << 1);
        const WATCHDOG = (1 << 2);
        const BROWNOUT = (1 << 3);
        // const RESERVED = (1 << 4);
        // const RESERVED = (1 << 5);
        // const RESERVED = (1 << 6);
        const OTHER = (1 << 7);
    }
}

/// Get last reset cause from hardware registers
fn reset_flags() -> LastResetCause {
    // safe to steal since we're only touching flags
    let csr = unsafe { &pac::Peripherals::steal().RCC.csr };

    // get flag states from register.
    let hardware = csr.read().pinrstf().bit();
    let software = csr.read().sftrstf().bit();
    let brownout = csr.read().borrstf().bit();
    let watchdog =
        csr.read().wwdgrstf().bit() || csr.read().iwdgrstf().bit();
    let other = csr.read().lpwrstf().bit()
        || csr.read().oblrstf().bit()
        || csr.read().firewallrstf().bit();

    // set flags
    let mut reset_flags = LastResetCause::empty();
    reset_flags.set(LastResetCause::HARDWARE, hardware);
    reset_flags.set(LastResetCause::SOFTWARE, software);
    reset_flags.set(LastResetCause::WATCHDOG, watchdog);
    reset_flags.set(LastResetCause::BROWNOUT, brownout);
    reset_flags.set(LastResetCause::OTHER, other);

    // print debugging information
    defmt::debug!(
        "\nlast reset cause:\n\
        \thardware: {}\n\
        \tsoftware: {}\n\
        \tbrownout: {}\n\
        \twatchdog: {}\n\
        \tother:    {}\n\
        ",
        hardware,
        software,
        brownout,
        watchdog,
        other,
    );

    // clear reset flags
    csr.write(|w| w.rmvf().set_bit());

    reset_flags
}
