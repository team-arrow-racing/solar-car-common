//! Standardised power-on reset messaging.
//!
//! This functionality is inspired by [GSFC-STD-1000](https://lws.larc.nasa.gov/pdf_files/3.11%20GSFC-STD-1000Revg_signature.pdf)
//! Rule 2.26:
//! > A power-on reset occurrence shall be unambiguously identifiable via
//! > telemetry.

use crate::com::{GroupExtension, MessageFormat};
use crate::device::{source_address, Device};
use bitflags::bitflags;
use bxcan::{ExtendedId, Frame};
use j1939::pgn::{Number, Pgn};

static PGN_STARTUP: Number = Number {
    specific: GroupExtension::Default as u8,
    format: MessageFormat::Startup as u8,
    data_page: false,
    extended_data_page: false,
};

/// Construct a message to be sent on device initialisation.
pub fn message(device: Device) -> Frame {
    let id = j1939::ExtendedId {
        priority: 3,
        pgn: Pgn::new(PGN_STARTUP),
        source_address: source_address(device).unwrap(),
    };

    Frame::new_data(
        ExtendedId::new(id.to_bits()).unwrap(),
        #[cfg(feature = "stm32l4")]
        [reset_flags().bits()],
        #[cfg(not(feature = "stm32l4"))]
        [],
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

#[cfg(feature = "stm32l4")]
/// Get last reset cause from hardware registers
fn reset_flags() -> LastResetCause {
    // safe to steal since we're only reading flags
    let csr = unsafe { &stm32l4xx_hal::pac::Peripherals::steal().RCC.csr };

    // get flag states from register.
    let hardware = csr.read().pinrstf().bit();
    let software = csr.read().sftrstf().bit();
    let brownout = csr.read().borrstf().bit();
    let watchdog = csr.read().wwdgrstf().bit() || csr.read().iwdgrstf().bit();
    let other =
        csr.read().lpwrstf().bit() || csr.read().oblrstf().bit() || csr.read().firewallrstf().bit();

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
