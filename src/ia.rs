//! Interface Association Descriptors

use core::num::NonZeroU8;

use crate::desc;

/// Interface Association Descriptor
pub struct Descriptor {
    // bLength: u8,
    // bDescriptorType: u8,
    /// Interface number of the first interface associated with this function
    pub bFirstInterface: u8,
    /// Number of contiguous interfaces associated to this functio
    pub bInterfaceCount: NonZeroU8,
    /// Class code
    pub bFunctionClass: NonZeroU8,
    /// Subclass code
    pub bFunctionSubClass: u8,
    /// Protocol code
    pub bFunctionProtocol: u8,
    /// Index of string descriptor describing this function
    pub iFunction: Option<NonZeroU8>,
}

impl Descriptor {
    /// The size of this descriptor in bytes
    pub const SIZE: u8 = 8;

    /// Returns the byte representation of this descriptor
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            desc::Type::InterfaceAssociation as u8,
            self.bFirstInterface,
            self.bInterfaceCount.get(),
            self.bFunctionClass.get(),
            self.bFunctionSubClass,
            self.bFunctionProtocol,
            self.iFunction.map(|nz| nz.get()).unwrap_or(0),
        ]
    }
}
