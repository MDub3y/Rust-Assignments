/*
  Problem 47: Serialization — Manual to_bytes / from_bytes

  Define a struct Record { id: u32, value: u16 }. Implement methods
  to_bytes(&self) -> Vec<u8> and from_bytes(data: &[u8]) -> Result<Self, String>
  using little-endian byte order. The serialized format should be
  [id: 4 bytes][value: 2 bytes] = 6 bytes total.

  Run the tests for this problem with:
    cargo test --test serialization_test
*/

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub value: u16,
}

impl Record {
    pub fn to_bytes(&self) -> Vec<u8> {
        let id_bytes = self.id.to_le_bytes();
        let value_bytes = self.value.to_le_bytes();

        let mut bytes = Vec::with_capacity(6);
        bytes.extend_from_slice(&id_bytes);
        bytes.extend_from_slice(&value_bytes);

        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 6 {
            return Err(format!("Expected 6 bytes, got: {}", data.len()));
        }

        let id_arr: [u8; 4] = data[0..4].try_into().map_err(|_| "Failed to extract ID".to_string())?;
        let id = u32::from_le_bytes(id_arr);

        let val_arr: [u8; 2] = data[4..6].try_into()
            .map_err(|_| "Failed to extract Value".to_string())?;
        let value = u16::from_le_bytes(val_arr);

        Ok(Record { id, value })
    }
}
