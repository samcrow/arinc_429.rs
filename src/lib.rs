//!
//! Common types for ARINC 429 communication
//!
//! # Serialization/Deserialization
//!
//! When compiled with the `serde` feature, all types support serialization and deserialization.
//!

#![doc(html_root_url = "https://docs.rs/arinc_429/0.1.0")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

/// An ARINC 429 message
///
/// The bits of a message are represented exactly as transmitted on the wires, with the least
/// significant bit transmitted first.
///
/// The label field is in the 8 least significant bits. Because the most significant digit of the
/// label is transmitted first, the label field is in the reverse of the usual bit order.
///
/// The parity bit is the most significant bit.
///
/// # Conversions
///
/// The `u32::from(Message)` and `Message::from(u32)` `From` implementations copy bits with no
/// changes.
///
/// Some ARINC 429 adapters use a different representation, where the bits of the label field are
/// reversed from their on-wire representation. The methods `Message::from_bits_label_swapped()` and
/// `Message::bits_label_swapped()` implement this conversion.
///
/// Conversions never panic.
///
/// # Examples
///
/// Create a message
///
/// ```
/// use arinc_429::Message;
/// let message = Message::from(0x10000056);
/// assert_eq!(0x10000056, u32::from(message));
/// ```
///
/// Label bit swapping
///
/// ```
/// use arinc_429::Message;
/// let message = Message::from_bits_label_swapped(0x10000056);
/// assert_eq!(0x1000006a, u32::from(message));
/// ```
///
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message(u32);

impl Message {
    /// Returns the bits that represent this message
    pub fn bits(&self) -> u32 {
        self.0
    }

    /// Returns the bits of this message, but
    /// with the order of the 8 label bits reversed.
    pub fn bits_label_swapped(&self) -> u32 {
        let bits = self.bits();
        Self::swap_label_bits(bits)
    }

    /// Creates a message from a message representation with the 8 label bits
    /// reversed. The returned Msg429 will be represented as transmitted on the wires.
    pub fn from_bits_label_swapped(bits: u32) -> Self {
        let bits = Self::swap_label_bits(bits);
        Message(bits)
    }

    /// Reverses the order of the 8 least significant bits of a value.
    /// Returns bits 32-9 unmodified, but with bits 1-8 reversed.
    fn swap_label_bits(bits: u32) -> u32 {
        let label = bits & 0xff;
        let new_label = ((label & 0x1) << 7) | ((label & 0x2) << 5) | ((label & 0x4) << 3) |
            ((label & 0x8) << 1) |
            ((label & 0x10) >> 1) | ((label & 0x20) >> 3) |
            ((label & 0x40) >> 5) | ((label & 0x80) >> 7);
        (bits & 0xffffff00) | new_label
    }
}

impl From<u32> for Message {
    /// Creates a message from bits as transmitted, with no modifications
    fn from(bits: u32) -> Self {
        Message(bits)
    }
}
impl From<Message> for u32 {
    /// Converts a message into bits, with no modifications
    fn from(Message(bits): Message) -> u32 {
        bits
    }
}

mod msg_fmt {
    use super::Message;

    #[cfg(feature = "std")]
    use std as fmt_base;
    #[cfg(not(feature = "std"))]
    use core as fmt_base;

    use self::fmt_base::fmt::{Debug, Formatter, Result};

    impl Debug for Message {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "Msg429({:#x})", self.0)
        }
    }
}

/// ARINC 429 communication speeds
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "speed"))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Speed {
    /// High speed, 100 kbps
    High,
    /// Low speed, 12.5 kbps
    Low,
}
