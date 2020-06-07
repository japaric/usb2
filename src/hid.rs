//! Human Interface Device (HID)
//!
//! For more details see (HID1.11)

use core::num::NonZeroU8;

use crate::bmrequesttype::{bmRequestType, Direction, Recipient};

/// HID specific requests
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Request {
    /// Interface index
    pub interface: u8,
    /// Kind of request
    pub kind: Kind,
}

/// HID request kind
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Silences a particular report until the specified time passes
    SetIdle {
        /// LSB = 4 milliseconds; `None` means "for an indefinite time"
        duration: Option<NonZeroU8>,
        /// ID of the report to silence; `None` means all reports
        report_id: Option<NonZeroU8>,
    },
    /// GET_DESCRIPTOR
    GetDescriptor {
        /// Length of the descriptor
        length: u16,
        /// The descriptor type
        descriptor: GetDescriptor,
    },
}

/// GET_DESCRIPTOR descriptor type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GetDescriptor {
    /// Report descriptor
    Report {
        /// Report descriptor index
        index: u8,
    },
}

const DESC_TYPE_HID: u8 = 0x21;
const DESC_TYPE_REPORT: u8 = 0x22;

impl Request {
    pub(crate) fn parse2(
        bmRequestType {
            direction,
            recipient,
            // ty must be `Standard` or `Class`
            ..
        }: bmRequestType,
        brequest: u8,
        wvalue: u16,
        windex: u16,
        wlength: u16,
    ) -> Result<Self, ()> {
        // bRequest
        const SET_IDLE: u8 = 10;
        const GET_DESCRIPTOR: u8 = 6;

        if brequest == SET_IDLE
            && recipient == Recipient::Interface
            && direction == Direction::HostToDevice
            && wlength == 0
        {
            let duration = NonZeroU8::new((wvalue >> 8) as u8);
            let report_id = NonZeroU8::new(wvalue as u8);
            let interface = crate::windex2interface(windex)?;

            Ok(Request {
                interface,
                kind: Kind::SetIdle {
                    duration,
                    report_id,
                },
            })
        } else if brequest == GET_DESCRIPTOR
            && recipient == Recipient::Interface
            && direction == Direction::DeviceToHost
        {
            let desc_ty = (wvalue >> 8) as u8;
            let index = wvalue as u8;
            let interface = crate::windex2interface(windex)?;
            let length = wlength;

            if desc_ty == DESC_TYPE_REPORT {
                Ok(Request {
                    interface,
                    kind: Kind::GetDescriptor {
                        length,
                        descriptor: GetDescriptor::Report { index },
                    },
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

/// Human Interface Device Class
pub struct Class;

impl Class {
    /// Class byte
    pub fn class(&self) -> NonZeroU8 {
        unsafe { NonZeroU8::new_unchecked(3) }
    }

    /// SubClass byte
    pub fn subclass(&self) -> u8 {
        0
    }

    /// Protocol byte
    pub fn protocol(&self) -> u8 {
        0
    }
}

/// HID descriptor -- single Report descriptor
pub struct Descriptor {
    /// Country code of the localized hardware
    pub bCountryCode: Country,

    /// Length of the report descriptor
    pub wDescriptorLength: u16,
}

/// Country code
#[derive(Clone, Copy, PartialEq)]
pub enum Country {
    /// Not Supported
    NotSupported = 0,
    /// Arabic
    Arabic = 1,
    /// Belgian
    Belgian = 2,
    /// Canadian-Bilingual
    CanadianBilingual = 3,
    /// Canadian-French
    CanadianFrench = 4,
    /// Czech Republic
    CzechRepublic = 5,
    /// Danish
    Danish = 6,
    /// Finnish
    Finnish = 7,
    /// French
    French = 8,
    /// German
    German = 9,
    /// Greek
    Greek = 10,
    /// Hebrew
    Hebrew = 11,
    /// Hungary
    Hungary = 12,
    /// International (ISO)
    InternationalISO = 13,
    /// Italian
    Italian = 14,
    /// Japan (Katakana)
    JapanKatakana = 15,
    /// Korean
    Korean = 16,
    /// Latin American
    LatinAmerican = 17,
    /// Netherlands/Dutch
    NetherlandsDutch = 18,
    /// Norwegian
    Norwegian = 19,
    /// Persian (Farsi)
    PersianFarsi = 20,
    /// Poland
    Poland = 21,
    /// Portuguese
    Portuguese = 22,
    /// Russia
    Russia = 23,
    /// Slovakia
    Slovakia = 24,
    /// Spanish
    Spanish = 25,
    /// Swedish
    Swedish = 26,
    /// Swiss/French
    SwissFrench = 27,
    /// Swiss/German
    SwissGerman = 28,
    /// Switzerland
    Switzerland = 29,
    /// Taiwan
    Taiwan = 30,
    /// Turkish-Q
    TurkishQ = 31,
    /// UK
    Uk = 32,
    /// US
    Us = 33,
    /// Yugoslavia
    Yugoslavia = 34,
    /// Turkish-F
    TurkishF = 35,
}

#[allow(non_upper_case_globals)]
const bcdHID: u16 = 0x01_00;

impl Descriptor {
    /// The size of this descriptor on the wire
    pub const SIZE: u8 = 9;

    /// Returns the wire representation of this descriptor
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            DESC_TYPE_HID,
            bcdHID as u8,
            (bcdHID >> 8) as u8,
            self.bCountryCode as u8,
            1,
            DESC_TYPE_REPORT,
            self.wDescriptorLength as u8,
            (self.wDescriptorLength >> 8) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::Request;

    #[test]
    fn get_descriptor() {
        assert_eq!(
            Request::parse(129, 6, 8704, 2, 64),
            Ok(Request::Hid(super::Request {
                interface: 2,
                kind: super::Kind::GetDescriptor {
                    length: 64,
                    descriptor: super::GetDescriptor::Report { index: 0 },
                }
            }))
        );
    }
}
