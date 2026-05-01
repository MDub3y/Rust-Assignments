/*
  Problem 60: Solana-Style Instruction — Unpack

  Simulate unpacking a Solana instruction. Define an enum Instruction with variants
  Initialize, Mint { amount: u64 }, and Transfer { amount: u64 }.
  Write a function unpack(data: &[u8]) -> Result<Instruction, String>.
  Data format: [tag: 1 byte][data: remaining bytes LE].
  Tags: 0 = Initialize, 1 = Mint, 2 = Transfer.

  Run the tests for this problem with:
    cargo test --test solana_instruction_test
*/

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Initialize,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}

pub fn unpack(data: &[u8]) -> Result<Instruction, String> {
    let (tag, rest) = data.split_first().ok_or("Empty instruction data")?;

    match tag {
        0 => Ok(Instruction::Initialize),
        1 => {
            let amount = unpack_u64(rest)?;
            Ok(Instruction::Mint { amount })
        },
        2 => {
            let amount = unpack_u64(rest)?;
            Ok(Instruction::Transfer { amount })
        },
        _ => Err("Invalid instruction tag".to_string()),
    }
}

fn unpack_u64(input: &[u8]) -> Result<u64, String> {
    if input.len() < 8 {
        return Err("Instruction data too short for u64".to_string());
    }

    let bytes: [u8; 8] = input[0..8].try_into().map_err(|_| "Failed to extract bytes")?;
    Ok(u64::from_le_bytes(bytes))
}