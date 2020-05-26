use core::num::NonZeroU8;

use crate::{desc, StringIndex};

/// Device descriptor
pub struct Descriptor {
    // pub blength: u8,
    // pub bDescriptorType: u8,
    // pub bcdUSB: u16,
    /// Device class
    pub bDeviceClass: u8,
    /// Device subclass
    pub bDeviceSubClass: u8,
    /// Device protocol
    pub bDeviceProtocol: u8,
    /// Maximum packet size
    pub bMaxPacketSize0: bMaxPacketSize0,
    /// Vendor ID
    pub idVendor: u16,
    /// Product ID
    pub idProduct: u16,
    /// Device release number
    pub bcdDevice: u16,
    /// Manufacturer string index
    pub iManufacturer: Option<StringIndex>,
    /// Product string index
    pub iProduct: Option<StringIndex>,
    /// Serial number string index
    pub iSerialNumber: Option<StringIndex>,
    /// Number of configurations
    pub bNumConfigurations: NonZeroU8,
}

#[allow(non_upper_case_globals)]
const bcdUSB: u16 = 0x0200; // 2.0

/// Maximum packet size
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum bMaxPacketSize0 {
    /// 8 bytes
    B8 = 8,
    /// 16 bytes
    B16 = 16,
    /// 32 bytes
    B32 = 32,
    /// 64 bytes
    B64 = 64,
}

impl Descriptor {
    /// The size of this descriptor on the wire
    pub const SIZE: u8 = 18;

    /// Returns the wire representation of this device endpoint
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            desc::Type::Device as u8,
            bcdUSB as u8,
            (bcdUSB >> 8) as u8,
            self.bDeviceClass,
            self.bDeviceSubClass,
            self.bDeviceProtocol,
            self.bMaxPacketSize0 as u8,
            self.idVendor as u8,
            (self.idVendor >> 8) as u8,
            self.idProduct as u8,
            (self.idProduct >> 8) as u8,
            self.bcdDevice as u8,
            (self.bcdDevice >> 8) as u8,
            self.iManufacturer.map(|nz| nz.get()).unwrap_or(0),
            self.iProduct.map(|nz| nz.get()).unwrap_or(0),
            self.iSerialNumber.map(|nz| nz.get()).unwrap_or(0),
            self.bNumConfigurations.get(),
        ]
    }
}
