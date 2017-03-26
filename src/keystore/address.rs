//! Custom JSON format for account addresses (without '0x' prefix)

use address::Address;
use regex::Regex;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::str::FromStr;

impl Encodable for Address {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(&self.to_string()[2..]) /* cut '0x' prefix */
    }
}

impl Decodable for Address {
    fn decode<D: Decoder>(d: &mut D) -> Result<Address, D::Error> {
        d.read_str().map(|s| format!("0x{}", s))
            .and_then(|s| Address::from_str(&s).map_err(|e| d.error(&e.to_string())))
    }
}

/// Try to extract `Address` from JSON formatted text
pub fn try_extract_address(text: &str) -> Option<Address> {
    lazy_static! {
        static ref ADDR_RE: Regex = Regex::new(r#"address.+([a-fA-F0-9]{40})"#).unwrap();
    }

    ADDR_RE.captures(text)
        .and_then(|g| g.get(1).map(|m| format!("0x{}", m.as_str())))
        .and_then(|s| s.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::try_extract_address;
    use address::Address;
    use rustc_serialize::json;

    #[test]
    fn should_encode_zero_address() {
        assert_eq!(json::encode(&Address::default()).unwrap(),
                   "\"0000000000000000000000000000000000000000\"");
    }

    #[test]
    fn should_encode_real_address() {
        let addr = Address::new([0x0e, 0x7c, 0x04, 0x51, 0x10, 0xb8, 0xdb, 0xf2, 0x97, 0x65,
                                 0x04, 0x73, 0x80, 0x89, 0x89, 0x19, 0xc5, 0xcb, 0x56, 0xf4]);

        assert_eq!(json::encode(&addr).unwrap(), "\"0e7c045110b8dbf29765047380898919c5cb56f4\"");
    }

    #[test]
    fn should_decode_zero_address() {
        assert_eq!(json::decode::<Address>("\"0000000000000000000000000000000000000000\"")
                       .unwrap(),
                   Address::default());
    }

    #[test]
    fn should_decode_real_address() {
        let addr = Address::new([0x0e, 0x7c, 0x04, 0x51, 0x10, 0xb8, 0xdb, 0xf2, 0x97, 0x65,
                                 0x04, 0x73, 0x80, 0x89, 0x89, 0x19, 0xc5, 0xcb, 0x56, 0xf4]);

        assert_eq!(json::decode::<Address>("\"0e7c045110b8dbf29765047380898919c5cb56f4\"")
                       .unwrap(),
                   addr);
    }

    #[test]
    fn should_not_decode_wrong_address() {
        assert!(json::decode::<Address>("\"__7c045110b8dbf29765047380898919c5cb56f4\"").is_err());
    }

    #[test]
    fn should_not_decode_not_string_address() {
        assert!(json::decode::<Address>("1234567890").is_err());
    }

    #[test]
    fn should_not_decode_empty_address() {
        assert!(json::decode::<Address>("\"\"").is_err());
    }

    #[test]
    fn should_extract_address() {
        assert_eq!(try_extract_address(r#"address: '008aeeda4d805471df9b2a5b0f38a0c3bcba786b',"#),
                   Some("0x008aeeda4d805471df9b2a5b0f38a0c3bcba786b".parse::<Address>().unwrap()));

        assert_eq!(try_extract_address(r#""address": "0047201aed0b69875b24b614dda0270bcd9f11cc","#),
                   Some("0x0047201aed0b69875b24b614dda0270bcd9f11cc".parse::<Address>().unwrap()));

        assert_eq!(try_extract_address(r#"  },
                     "address": "3f4e0668c20e100d7c2a27d4b177ac65b2875d26",
                     "name": "",
                     "meta": "{}"
                   }"#),
                   Some("0x3f4e0668c20e100d7c2a27d4b177ac65b2875d26".parse::<Address>().unwrap()));
    }

    #[test]
    fn should_ignore_pointless() {
        assert_eq!(try_extract_address("\"version\": 3"), None);
    }

    #[test]
    fn should_ignore_empty() {
        assert_eq!(try_extract_address(""), None);
    }
}
