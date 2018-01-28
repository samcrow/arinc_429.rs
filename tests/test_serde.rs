//! Tests serializing and deserializing

extern crate arinc_429;


#[cfg(feature = "serde")]
extern crate serde;
extern crate serde_test;

#[cfg(feature = "serde")]
mod with_serde {
    use serde_test::{Token, assert_tokens};
    use arinc_429::Speed;
    use arinc_429::Message;

    #[test]
    fn test_low_speed() {
        let speed = Speed::Low;
        assert_tokens(&speed, &[
            Token::UnitVariant {
                name: "speed",
                variant: "low",
            }
        ]);
    }

    #[test]
    fn test_high_speed() {
        let speed = Speed::High;
        assert_tokens(&speed, &[
            Token::UnitVariant {
                name: "speed",
                variant: "high",
            }
        ]);
    }

    #[test]
    fn test_message_zero() {
        let message = Message::from(0x0);
        assert_tokens(&message, &[
            Token::NewtypeStruct { name: "Message" },
            Token::U32(0x0),
        ]);
    }

    #[test]
    fn test_message_nonzero() {
        let message = Message::from(0xface1234);
        assert_tokens(&message, &[
            Token::NewtypeStruct { name: "Message" },
            Token::U32(0xface1234),
        ]);
    }
}
