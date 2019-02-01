//!
//! Common types for ARINC 429 communication
//!
//! # Serialization/Deserialization
//!
//! When compiled with the `serde` feature, all types support serialization and deserialization.
//!

#![doc(html_root_url = "https://docs.rs/arinc_429/0.1.5")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "std")]
use std as base;
#[cfg(not(feature = "std"))]
use core as base;

mod parity_error;
pub use self::parity_error::ParityError;

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
/// # use arinc_429::Message;
/// let message = Message::from(0x10000056);
/// assert_eq!(0x10000056, u32::from(message));
/// ```
///
/// Label bit swapping
///
/// ```
/// # use arinc_429::Message;
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
    /// reversed. The returned Message will be represented as transmitted on the wires.
    pub fn from_bits_label_swapped(bits: u32) -> Self {
        let bits = Self::swap_label_bits(bits);
        Message(bits)
    }

    /// Checks the parity of this message, and returns an error if the parity is not odd
    ///
    /// # Examples
    ///
    /// ```
    /// # use arinc_429::Message;
    /// assert!(Message::from(0x0).check_parity().is_err());
    /// assert!(Message::from(0xf03ccccc).check_parity().is_err());
    /// assert!(Message::from(0x1).check_parity().is_ok());
    /// assert!(Message::from(0xf13ccccc).check_parity().is_ok());
    /// ```
    ///
    pub fn check_parity(&self) -> Result<(), ParityError> {
        // Should have an odd number of ones
        if self.0.count_ones() % 2 == 1 {
            Ok(())
        } else {
            let parity = (self.0 >> 31) as u8;
            let expected = parity ^ 1;
            Err(ParityError::new(expected, parity))
        }
    }

    /// Calculates the parity of this message and returns a new message with the parity bit (31) to
    /// the correct value
    ///
    /// # Examples
    ///
    /// ```
    /// # use arinc_429::Message;
    /// // Create a message with incorrect (even) parity
    /// let message = Message::from(0x22443300);
    /// assert_eq!(message.update_parity().bits(), 0xa2443300);
    /// ```
    ///
    /// ```
    /// # use arinc_429::Message;
    /// // Create a message with correct (odd) parity
    /// let message = Message::from(0x22443301);
    /// // Message should not change
    /// assert_eq!(message.update_parity(), message);
    /// ```
    ///
    pub fn update_parity(&self) -> Message {
        match self.check_parity() {
            Ok(_) => self.clone(),
            Err(_) => {
                // Flip parity bit
                Message(self.0 ^ 1 << 31)
            }
        }
    }

    /// Returns the label of this message. This can be used to show the label as an octal number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use arinc_429::Message;
    /// let message = Message::from(0x84000109);
    /// let label = message.label();
    /// assert_eq!(format!("{:?}", label), "Label(0o220)");
    /// ```
    ///
    /// ```
    /// # use arinc_429::Message;
    /// let message = Message::from(0x84000180);
    /// let label = message.label();
    /// assert_eq!(format!("{:?}", label), "Label(0o001)");
    /// ```
    ///
    pub fn label(&self) -> Label {
        let swapped = Self::swap_label_bits(self.0);
        Label(swapped as u8)
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

/// A label from a message, displayed in normal bit ordering as an octal number
// Implementation detail: The enclosed label is stored in normal bit ordering
pub struct Label(u8);

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
    use super::{Message, Label};

    use base::fmt::{Debug, Formatter, Result};

    impl Debug for Message {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "Message({:#010x})", self.0)
        }
    }

    impl Debug for Label {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "Label({:#05o})", self.0)
        }
    }

    #[cfg(all(test, feature = "std"))]
    mod message_test {
        use super::super::Message;

        #[test]
        fn test_zero() {
            let message = Message::from(0x0);
            check_debug(&message, "Message(0x00000000)");
        }

        #[test]
        fn test_all_ones() {
            let message = Message::from(0xffffffff);
            check_debug(&message, "Message(0xffffffff)");
        }

        fn check_debug(message: &Message, expected: &str) {
            let actual = format!("{:?}", message);
            assert_eq!(actual, expected, "Incorrect debug representation");
        }
    }

    #[cfg(all(test, feature = "std"))]
    mod label_test {
        use super::super::Label;

        #[test]
        fn test_zero() {
            let label = Label(0x0);
            check_debug(&label, "Label(0o000)");
        }

        #[test]
        fn test_all_ones() {
            let label = Label(0xff);
            check_debug(&label, "Label(0o377)");
        }

        fn check_debug(label: &Label, expected: &str) {
            let actual = format!("{:?}", label);
            assert_eq!(actual, expected, "Incorrect debug representation");
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
