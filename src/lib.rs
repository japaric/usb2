//! USB 2.0 data types
//!
//! # References
//!
//! - (USB2) Universal Serial Bus Specification Revision 2.0 (April 27, 2000)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![deny(missing_docs)] // TODO
#![no_std]

use core::num::NonZeroU8;

#[macro_use]
mod macros;

mod bmrequesttype;
mod brequest;
pub mod configuration;
mod desc;
pub mod device;
mod feature;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Default,
    Address(Address),
    Configured { address: Address, value: NonZeroU8 },
}

/// Device address assigned by the host; will be in the range 1..=127
pub type Address = NonZeroU8;

pub type StringIndex = NonZeroU8;

/// Endpoint address
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Endpoint {
    pub direction: Direction,
    pub number: u8,
}

/// Direction from the point of view of the host
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Device to Host
    In,
    /// Host to Device
    Out,
}

#[cfg(TODO)]
pub enum Request {
    Standard(StandardRequest),
    // TODO Class-specific requests
}

/// Standard device requests
///
/// See section 9.4 of (USB2)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StandardRequest {
    /// CLEAR_FEATURE
    ClearFeature(ClearFeature),
    /// GET_CONFIGURATION
    GetConfiguration,
    /// GET_DESCRIPTOR
    GetDescriptor {
        descriptor: GetDescriptor,
        length: u16,
    },
    /// GET_INTERFACE
    GetInterface { interface: u8 },
    /// GET_STATUS
    GetStatus(GetStatus),
    /// SET_ADDRESS
    SetAddress { address: Option<Address> },
    /// SET_CONFIGURATION
    SetConfiguration { value: Option<NonZeroU8> },
    /// SET_DESCRIPTOR
    SetDescriptor {
        descriptor: SetDescriptor,
        length: u16,
    },
    /// SET_FEATURE
    SetFeature(SetFeature),
    /// SET_INTERFACE
    SetInterface { interface: u8, alternate: u8 },
    /// SYNCH_FRAME
    SynchFrame { endpoint: Endpoint },
}

/// GET_DESCRIPTOR descriptor
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GetDescriptor {
    Configuration { index: u8 },
    Device,
    DeviceQualifier,
    OtherSpeedConfiguration { index: u8 },
    String { index: u8, lang_id: u16 },
}

/// SET_DESCRIPTOR descriptor
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SetDescriptor {
    Configuration { index: u8 },
    Device,
    String { index: u8, lang_id: u16 },
}

const MAX_ADDRESS: u16 = 127;

/// CLEAR_FEATURE feature selector
///
/// See table 9-6 of (USB2)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClearFeature {
    DeviceRemoteWakeup,
    EndpointHalt { endpoint: Endpoint },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GetStatus {
    Device,
    Endpoint(Endpoint),
    Interface(u8),
}

/// SET_FEATURE feature selector
///
/// See table 9-6 of (USB2)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SetFeature {
    DeviceRemoteWakeup,
    EndpointHalt { endpoint: Endpoint },
    TestMode { test: Test },
}

repr!(u8,
      /// Test selector
      Test {
    /// Test_J
    J = 0x01,
    /// Test_K
    K = 0x02,
    /// Test_SE0_NAK
    SE0_NAK = 0x03,
    /// Test_Packet
    Packet = 0x04,
    /// Test_Force_Enable
    ForceEnable = 0x05,
});

impl StandardRequest {
    /// Parses a standard request
    pub fn parse(
        bmrequesttype: u8,
        brequest: u8,
        wvalue: u16,
        windex: u16,
        wlength: u16,
    ) -> Result<Self, ()> {
        use bmrequesttype::{bmRequestType, Direction, Recipient};

        let bmRequestType {
            direction,
            recipient,
            ty,
        } = bmRequestType::parse(bmrequesttype)?;
        if ty != bmrequesttype::Type::Standard {
            return Err(());
        }

        // See table 9-3 of (USB2)
        match (brequest, direction) {
            // see section 9.4.1 of (USB2)
            (brequest::CLEAR_FEATURE, Direction::HostToDevice)
                if recipient != Recipient::Other && wlength == 0 =>
            {
                if wvalue == feature::DEVICE_REMOTE_WAKEUP
                    && recipient == Recipient::Device
                    && windex == 0
                {
                    Ok(StandardRequest::ClearFeature(
                        ClearFeature::DeviceRemoteWakeup,
                    ))
                } else if wvalue == feature::ENDPOINT_HALT && recipient == Recipient::Endpoint {
                    Ok(StandardRequest::ClearFeature(ClearFeature::EndpointHalt {
                        endpoint: windex2endpoint(windex)?,
                    }))
                } else {
                    Err(())
                }
            }

            // see section 9.4.2 of (USB2)
            (brequest::GET_CONFIGURATION, Direction::DeviceToHost)
                if recipient == Recipient::Device && wvalue == 0 && windex == 0 && wlength == 1 =>
            {
                Ok(StandardRequest::GetConfiguration)
            }

            // see section 9.4.3 of (USB2)
            (brequest::GET_DESCRIPTOR, Direction::DeviceToHost)
                if recipient == Recipient::Device =>
            {
                let desc_ty = (wvalue >> 8) as u8;
                let desc_idx = wvalue as u8;

                let ty = desc::Type::_from(desc_ty).ok_or(())?;

                let desc = match ty {
                    desc::Type::Device if desc_idx == 0 && windex == 0 => GetDescriptor::Device,
                    desc::Type::DeviceQualifier if desc_idx == 0 && windex == 0 => {
                        GetDescriptor::DeviceQualifier
                    }
                    desc::Type::Configuration if windex == 0 => {
                        GetDescriptor::Configuration { index: desc_idx }
                    }
                    desc::Type::OtherSpeedConfiguration if windex == 0 => {
                        GetDescriptor::OtherSpeedConfiguration { index: desc_idx }
                    }
                    desc::Type::String => GetDescriptor::String {
                        index: desc_idx,
                        lang_id: windex,
                    },
                    // other types cannot appear in a GET_DESCRIPTOR request
                    _ => return Err(()),
                };

                Ok(StandardRequest::GetDescriptor {
                    descriptor: desc,
                    length: wlength,
                })
            }

            // see section 9.4.4 of (USB2)
            (brequest::GET_INTERFACE, Direction::DeviceToHost)
                if recipient == Recipient::Interface && wvalue == 0 && wlength == 1 =>
            {
                Ok(StandardRequest::GetInterface {
                    interface: windex2interface(windex)?,
                })
            }

            // see section 9.4.5 of (USB2)
            (brequest::GET_STATUS, Direction::DeviceToHost) if wvalue == 0 && wlength == 2 => {
                let status = match recipient {
                    Recipient::Device if windex == 0 => GetStatus::Device,
                    Recipient::Endpoint => GetStatus::Endpoint(windex2endpoint(windex)?),
                    Recipient::Interface => GetStatus::Interface(windex2interface(windex)?),
                    _ => return Err(()),
                };

                Ok(StandardRequest::GetStatus(status))
            }

            // see section 9.4.6 of (USB2)
            (brequest::SET_ADDRESS, Direction::HostToDevice)
                if recipient == Recipient::Device
                    && windex == 0
                    && wlength == 0
                    && wvalue <= MAX_ADDRESS =>
            {
                let address = NonZeroU8::new(wvalue as u8);
                Ok(StandardRequest::SetAddress { address })
            }

            // see section 9.4.7 of (USB2)
            (brequest::SET_CONFIGURATION, Direction::HostToDevice)
                if recipient == Recipient::Device
                    && windex == 0
                    && wlength == 0
                    && wvalue >> 8 == 0 =>
            {
                Ok(StandardRequest::SetConfiguration {
                    value: NonZeroU8::new(wvalue as u8),
                })
            }

            (brequest::SET_DESCRIPTOR, Direction::HostToDevice)
                if recipient == Recipient::Device =>
            {
                let desc_ty = (wvalue >> 8) as u8;
                let desc_idx = wvalue as u8;

                let ty = desc::Type::_from(desc_ty).ok_or(())?;

                let desc = match ty {
                    desc::Type::Device if desc_idx == 0 && windex == 0 => SetDescriptor::Device,
                    desc::Type::Configuration if windex == 0 => {
                        SetDescriptor::Configuration { index: desc_idx }
                    }
                    desc::Type::String => SetDescriptor::String {
                        index: desc_idx,
                        lang_id: windex,
                    },
                    // other types cannot appear in a SET_DESCRIPTOR request
                    _ => return Err(()),
                };

                Ok(StandardRequest::SetDescriptor {
                    descriptor: desc,
                    length: wlength,
                })
            }

            (brequest::SET_FEATURE, Direction::HostToDevice) if wlength == 0 => {
                let feature = if wvalue == feature::DEVICE_REMOTE_WAKEUP
                    && recipient == Recipient::Device
                    && windex == 0
                {
                    SetFeature::DeviceRemoteWakeup
                } else if wvalue == feature::TEST_MODE
                    && recipient == Recipient::Device
                    && windex as u8 == 0
                {
                    SetFeature::TestMode {
                        test: Test::_from((windex >> 8) as u8).ok_or(())?,
                    }
                } else if wvalue == feature::ENDPOINT_HALT && recipient == Recipient::Endpoint {
                    SetFeature::EndpointHalt {
                        endpoint: windex2endpoint(windex)?,
                    }
                } else {
                    return Err(());
                };

                Ok(StandardRequest::SetFeature(feature))
            }

            (brequest::SET_INTERFACE, Direction::HostToDevice)
                if recipient == Recipient::Interface && wlength == 0 =>
            {
                let interface = windex2interface(windex)?;
                let alternate = windex2interface(wvalue)?;

                Ok(StandardRequest::SetInterface {
                    interface,
                    alternate,
                })
            }

            (brequest::SYNCH_FRAME, Direction::DeviceToHost)
                if recipient == Recipient::Endpoint && wvalue == 0 && wlength == 2 =>
            {
                Ok(StandardRequest::SynchFrame {
                    endpoint: windex2endpoint(windex)?,
                })
            }

            _ => Err(()),
        }
    }
}

fn windex2endpoint(windex: u16) -> Result<Endpoint, ()> {
    if windex >> 8 != 0 {
        return Err(());
    }

    let windex = windex as u8;
    let direction = windex >> 4;
    let direction = if direction == 0b0000 {
        Direction::Out
    } else if direction == 0b1000 {
        Direction::In
    } else {
        return Err(());
    };

    Ok(Endpoint {
        direction,
        number: windex & 0b1111,
    })
}

fn windex2interface(windex: u16) -> Result<u8, ()> {
    if windex >> 8 != 0 {
        Err(())
    } else {
        Ok(windex as u8)
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroU8;

    use crate::{Direction, Endpoint, GetDescriptor, StandardRequest};

    #[test]
    fn endpoint() {
        assert_eq!(
            crate::windex2endpoint(0x0080),
            Ok(Endpoint {
                direction: Direction::In,
                number: 0
            })
        );

        assert_eq!(
            crate::windex2endpoint(0x0000),
            Ok(Endpoint {
                direction: Direction::Out,
                number: 0
            })
        );

        assert!(crate::windex2endpoint(0x0010).is_err());
        assert!(crate::windex2endpoint(0x0090).is_err());
    }

    #[test]
    fn get_descriptor_device() {
        assert_eq!(
            StandardRequest::parse(0b1000_0000, 0x06, 0x01_00, 0, 18),
            Ok(StandardRequest::GetDescriptor {
                descriptor: GetDescriptor::Device,
                length: 18
            })
        );

        // wrong descriptor index
        assert!(StandardRequest::parse(0b1000_0000, 0x06, 0x01_01, 0, 18).is_err(),);

        // language ID
        assert!(StandardRequest::parse(0b1000_0000, 0x06, 0x01_00, 1033, 18).is_err(),);
    }

    #[test]
    fn get_descriptor_configuration() {
        // GET_DESCRIPTOR Configuration 0
        assert_eq!(
            StandardRequest::parse(0b1000_0000, 0x06, 0x02_00, 0, 9),
            Ok(StandardRequest::GetDescriptor {
                descriptor: GetDescriptor::Configuration { index: 0 },
                length: 9
            })
        );

        assert!(StandardRequest::parse(0b1000_0000, 0x06, 0x02_00, 1033, 9).is_err());
    }

    #[test]
    fn set_address() {
        // SET_ADDRESS 16
        assert_eq!(
            StandardRequest::parse(0b0000_0000, 0x05, 0x00_10, 0, 0),
            Ok(StandardRequest::SetAddress {
                address: NonZeroU8::new(16)
            })
        );

        // has language id but shouldn't
        assert!(StandardRequest::parse(0b0000_0000, 0x05, 0x00_10, 1033, 0).is_err());

        // length should be zero
        assert!(StandardRequest::parse(0b0000_0000, 0x05, 0x00_10, 0, 1).is_err());
    }

    #[test]
    fn set_configuration() {
        // SET_CONFIGURATION 1
        assert_eq!(
            StandardRequest::parse(0b0000_0000, 0x09, 0x00_01, 0, 0),
            Ok(StandardRequest::SetConfiguration {
                value: NonZeroU8::new(1)
            })
        );

        // has language id but shouldn't
        assert!(StandardRequest::parse(0b0000_0000, 0x09, 0x00_01, 1033, 0).is_err());

        // length should be zero
        assert!(StandardRequest::parse(0b0000_0000, 0x09, 0x00_01, 0, 1).is_err());
    }
}
