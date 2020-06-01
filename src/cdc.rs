//! Class Communication Device

use core::num::NonZeroU8;

pub mod acm;
pub mod call;
pub mod header;
pub mod union;

/// Communication Device
#[derive(Clone, Copy)]
pub enum Class {
    /// Communications Interface Class
    Communications {
        /// Communications class subclass
        subclass: SubClass,
        /// Communications class protocol
        protocol: Protocol,
    },

    /// Data interface class
    CdcData,
}

impl Class {
    /// Class byte
    pub fn class(&self) -> NonZeroU8 {
        unsafe {
            NonZeroU8::new_unchecked(match self {
                Class::Communications { .. } => 2,
                Class::CdcData => 10,
            })
        }
    }

    /// SubClass byte
    pub fn subclass(&self) -> u8 {
        match self {
            Class::Communications { subclass, .. } => *subclass as u8,
            Class::CdcData => 0,
        }
    }

    /// Protocol byte
    pub fn protocol(&self) -> u8 {
        match self {
            Class::Communications { protocol, .. } => *protocol as u8,
            Class::CdcData => 0,
        }
    }
}

/// Communications Class Subclass codes
#[derive(Clone, Copy)]
pub enum SubClass {
    /// Abstract Control Model
    AbstractControlModel = 0x02,
}

/// Communications Class Protocol codes
#[derive(Clone, Copy)]
pub enum Protocol {
    /// AT Commands
    ATCommands = 1,
}

const CS_INTERFACE: u8 = 0x24;

const SUBTYPE_HEADER: u8 = 0x00;
const SUBTYPE_CALL: u8 = 0x01;
const SUBTYPE_ACM: u8 = 0x02;
const SUBTYPE_UNION: u8 = 0x06;
