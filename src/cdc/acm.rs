//! Abstract Control Management functional descriptor

use crate::bmrequesttype::{bmRequestType, Direction, Recipient, Type};

/// ACM request
pub enum Request {
    /// GET_LINE_CODING
    GetLineCoding {
        /// Target interface
        interface: u8,
    },
    /// SET_LINE_CODING
    SetLineCoding {
        /// Target interface
        interface: u8,
    },
    /// SET_CONTROL_LINE_STATE
    SetControlLineState(SetControlLineState),
}

const SET_LINE_CODING: u8 = 0x20;
const GET_LINE_CODING: u8 = 0x21;
const SET_CONTROL_LINE_STATE: u8 = 0x22;

/// Line Coding structure
#[derive(Clone, Copy)]
pub struct LineCoding {
    /// Data terminal rate, in bits per second
    pub dwDTERate: u32,
    /// Stop bits
    pub bCharFormat: bCharFormat,
    /// Parity
    pub bParityType: bParityType,
    /// Data bits
    pub bDataBits: bDataBits,
}

impl LineCoding {
    /// The size of this structure on the wire
    pub const SIZE: u8 = 7;

    /// Returns the wire representation of this structure
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            self.dwDTERate as u8,
            (self.dwDTERate >> 8) as u8,
            (self.dwDTERate >> 16) as u8,
            (self.dwDTERate >> 24) as u8,
            self.bCharFormat as u8,
            self.bParityType as u8,
            self.bDataBits as u8,
        ]
    }
}

/// Stop bits
#[derive(Clone, Copy)]
pub enum bCharFormat {
    /// 1 stop bit
    Stop1 = 0,
    /// 1.5 stop bit
    Stop1_5 = 1,
    /// 2 stop bits
    Stop2 = 2,
}

/// Parity
#[derive(Clone, Copy)]
pub enum bParityType {
    /// None
    None = 0,
    /// Odd
    Odd = 1,
    /// Even
    Even = 2,
    /// Mark
    Mark = 3,
    /// Space
    Space = 4,
}

/// Data bits
#[derive(Clone, Copy)]
pub enum bDataBits {
    /// 5 bits
    _5 = 5,
    /// 6 bits
    _6 = 6,
    /// 7 bits
    _7 = 7,
    /// 8 bits
    _8 = 8,
    /// 16 bits
    _16 = 16,
}

/// SET_CONTROL_LINE_STATE request
pub struct SetControlLineState {
    /// Target interface
    pub interface: u8,
    /// DTE is present
    pub dte_present: bool,
    /// Carrier control for half-duplex modems. `true` = activate RTS carrier; `false` = deactivate
    pub rts: bool,
}

impl Request {
    /// Parses an ACM request
    pub fn parse(
        bmrequesttype: u8,
        brequest: u8,
        wvalue: u16,
        windex: u16,
        wlength: u16,
    ) -> Result<Self, ()> {
        let bmrequesttype = bmRequestType::parse(bmrequesttype)?;

        if bmrequesttype.ty != Type::Class {
            return Err(());
        }

        Self::parse2(bmrequesttype, brequest, wvalue, windex, wlength)
    }

    pub(crate) fn parse2(
        bmRequestType {
            direction,
            recipient,
            ..
        }: bmRequestType,
        brequest: u8,
        wvalue: u16,
        windex: u16,
        wlength: u16,
    ) -> Result<Self, ()> {
        match (brequest, direction) {
            (SET_LINE_CODING, Direction::HostToDevice)
                if recipient == Recipient::Interface && wvalue == 0 && wlength == 7 =>
            {
                let interface = crate::windex2interface(windex)?;

                Ok(Request::SetLineCoding { interface })
            }

            (GET_LINE_CODING, Direction::DeviceToHost)
                if recipient == Recipient::Interface && wvalue == 0 && wlength == 7 =>
            {
                let interface = crate::windex2interface(windex)?;

                Ok(Request::GetLineCoding { interface })
            }

            (SET_CONTROL_LINE_STATE, Direction::HostToDevice)
                if recipient == Recipient::Interface && wlength == 0 =>
            {
                let interface = crate::windex2interface(windex)?;
                if wvalue & !0b11 != 0 {
                    return Err(());
                }

                let dte_present = wvalue & 1 != 0;
                let rts = wvalue & (1 << 1) != 0;

                Ok(Request::SetControlLineState(SetControlLineState {
                    interface,
                    rts,
                    dte_present,
                }))
            }

            _ => Err(()),
        }
    }
}

/// Abstract Control Management functional descriptor
#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct Descriptor {
    // bFunctionLength: u8,
    // bDescriptorType: u8,
    // bDescriptorSubtype: u8,
    /// Capabilities
    pub bmCapabilities: Capabilities,
}

/// Capabilities
#[derive(Clone, Copy)]
pub struct Capabilities {
    /// Device supports `{Set,Clear,Get}_Comm_Feature`
    pub comm_features: bool,
    /// Device supports `{Set,Get}_Line_Coding`, `Set_Control_Line_State` and `Serial_State`
    pub line_serial: bool,
    /// Device supports `Send_Break`
    pub send_break: bool,
    /// Device supports `Network_Connection`
    pub network_connection: bool,
}

impl Capabilities {
    fn byte(&self) -> u8 {
        let mut byte = 0;
        if self.comm_features {
            byte |= 1 << 0;
        }
        if self.line_serial {
            byte |= 1 << 1;
        }
        if self.send_break {
            byte |= 1 << 2;
        }
        if self.network_connection {
            byte |= 1 << 3;
        }
        byte
    }
}

impl Descriptor {
    /// Size of this descriptor on the wire
    pub const SIZE: u8 = 4;

    /// Returns the wire representation of this device endpoint
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        [
            Self::SIZE,
            super::CS_INTERFACE,
            super::SUBTYPE_ACM,
            self.bmCapabilities.byte(),
        ]
    }
}
