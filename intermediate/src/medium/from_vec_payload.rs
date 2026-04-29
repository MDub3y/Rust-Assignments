/*
  Problem 61: Intermediate — From Vec to Payload

  Implement a struct Payload { data: Vec<u8> } that implements From<Vec<u16>>.
  The conversion should pack each u16 into two u8 bytes (big-endian).

  Run the tests for this problem with:
    cargo test --test from_vec_payload_test
*/

#[derive(Debug, PartialEq)]
pub struct Payload {
    pub data: Vec<u8>,
}

impl From<Vec<u16>> for Payload {
    fn from(v: Vec<u16>) -> Self {
        let mut data = Vec::with_capacity(v.len() * 2);

        for val in v {
            data.push((val >> 8) as u8);
            data.push((val & 0xFF) as u8);
        }

        Payload{ data }
    }
}
