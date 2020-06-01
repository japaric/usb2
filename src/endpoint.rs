//! Endpoint descriptors

use crate::{desc, Endpoint};

/// Endpoint descriptor
pub struct Descriptor {
    // pub bLength: u8,
    // pub bDescriptorType: u8,
    /// Endpoint address
    pub bEndpointAddress: Endpoint,
    /// Endpoint type
    pub ty: Type,
    /// Maximum packet size (must be less than `1 << 11`)
    pub max_packet_size: u16,
    /// Polling interval
    pub bInterval: u8,
}

/// End point type
#[derive(Clone, Copy)]
pub enum Type {
    /// Bulk endpoint
    Bulk,

    /// Control endpoint
    Control,

    /// Interrupt endpoint
    Interrupt {
        /// Transactions per microframe
        transactions_per_microframe: Transactions,
    },

    /// Isochronous endpoint
    Isochronous {
        /// Synchronization type
        synchronization_type: SynchronizationType,
        /// Usage type
        usage_type: UsageType,
        /// Transactions per microframe
        transactions_per_microframe: Transactions,
    },
}

impl Type {
    fn bmAttributes(&self) -> u8 {
        match self {
            Type::Bulk => 0b10,
            Type::Control => 0b00,
            Type::Interrupt { .. } => 0b11,
            Type::Isochronous {
                synchronization_type,
                usage_type,
                ..
            } => 0b01 | (*synchronization_type as u8) << 2 | (*usage_type as u8) << 4,
        }
    }
}

/// Synchronization type
#[derive(Clone, Copy)]
pub enum SynchronizationType {
    /// No synchronization
    NoSynchronization = 0b00,
    /// Asynchronous
    Asynchronous = 0b01,
    /// Adaptive
    Adaptive = 0b10,
    /// Synchronous
    Synchronous = 0b11,
}

/// Usage type
#[derive(Clone, Copy)]
pub enum UsageType {
    /// Data endpoint
    DataEndpoint = 0b00,
    /// Feedback endpoint
    FeedbackEndpoint = 0b01,
    /// Implicit feedback data endpoint
    ImplicitFeedbackDataEndpoint = 0b10,
}

/// Transactions per microframe
#[derive(Clone, Copy)]
pub enum Transactions {
    /// 1 transaction per microframe
    _1 = 0b00,
    /// 2 transactions per microframe
    _2 = 0b01,
    /// 3 transactions per microframe
    _3 = 0b10,
}

impl Descriptor {
    /// The size of this descriptor on the wire
    pub const SIZE: u8 = 7;

    /// Returns the wire representation of this descriptor
    pub fn bytes(&self) -> [u8; Self::SIZE as usize] {
        let mut word = self.max_packet_size & ((1 << 11) - 1);
        match self.ty {
            Type::Interrupt {
                transactions_per_microframe,
            }
            | Type::Isochronous {
                transactions_per_microframe,
                ..
            } => {
                word |= (transactions_per_microframe as u16) << 11;
            }
            _ => {}
        }

        [
            Self::SIZE,
            desc::Type::Endpoint as u8,
            self.bEndpointAddress.byte(),
            self.ty.bmAttributes(),
            word as u8,
            (word >> 8) as u8,
            self.bInterval,
        ]
    }
}
