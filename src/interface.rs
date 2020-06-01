//! Interface descriptors

use core::num::NonZeroU8;

use crate::desc;

/// Interface descriptor
///
/// See section 9.6.5 of (USB2)
pub struct Descriptor {
    // pub bLength: u8,
    // pub bDescriptorType: u8,
    /// Interface number
    pub bInterfaceNumber: u8,
    /// Alternative setting
    pub bAlternativeSetting: u8,
    /// Number of endpoints
    pub bNumEndpoints: u8,
    /// Interface class
    pub bInterfaceClass: u8,
    /// Interface subclass
    pub bInterfaceSubClass: u8,
    /// Interface protocol
    pub bInterfaceProtocol: u8,
    /// Interface string descriptor index
    pub iInterface: Option<NonZeroU8>,
}

impl Descriptor {
    /// The size of this descriptor in bytes
    pub const SIZE: u8 = 9;

    /// Returns the byte representation of this descriptor
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            desc::Type::Interface as u8,
            self.bInterfaceNumber,
            self.bAlternativeSetting,
            self.bNumEndpoints,
            self.bInterfaceClass,
            self.bInterfaceSubClass,
            self.bInterfaceProtocol,
            self.iInterface.map(|nz| nz.get()).unwrap_or(0),
        ]
    }
}
