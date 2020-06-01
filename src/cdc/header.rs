//! CDC Header descriptors

/// CDC Header functional descriptor
///
/// See section of 5.2.3.1 of (USBCDC1.2)
pub struct Descriptor {
    /// Communications Devices Specification release number (Binary-coded Decimal)
    pub bcdCDC: u16,
}

impl Descriptor {
    /// The size of this descriptor on the wire
    pub const SIZE: u8 = 5;

    /// Returns the wire representation of this device endpoint
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            super::CS_INTERFACE,
            super::SUBTYPE_HEADER,
            self.bcdCDC as u8,
            (self.bcdCDC >> 8) as u8,
        ]
    }
}
