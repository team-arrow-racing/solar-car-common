pub enum Device {
    VehicleController,
    SteeringWheel,
    ArrayIsolationController,
    WaveSculptor,
    MpptA,
    MpptB,
}

pub fn device_source_address(device: Device) -> Option<u8> {
    match device {
        Device::VehicleController => Some(0x10),
        Device::SteeringWheel => Some(0x20),
        Device::ArrayIsolationController => Some(0x30),
        _ => None
    }
}

pub fn device_base_address(device: Device) -> Option<u16> {
    match device {
        Device::WaveSculptor => Some(0x400),
        Device::MpptA => Some(0x600),
        Device::MpptB => Some(0x610),
        _ => None,
    }
}
