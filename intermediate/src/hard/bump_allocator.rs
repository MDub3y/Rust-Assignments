/*
  Problem 68: Bump Allocator — Simple Simulation

  Simulate a bump allocator. Define a struct BumpAllocator with a buffer [u8; 1024]
  and a cursor (next available byte). Implement alloc(size: usize) -> Result<&mut [u8], String>
  that returns a slice of the given size and advances the cursor.

  Run the tests for this problem with:
    cargo test --test bump_allocator_test
*/

pub struct BumpAllocator {
    pub buffer: [u8; 1024],
    pub cursor: usize,
}

impl BumpAllocator {
    pub fn new() -> Self {
        Self {
            buffer: [0; 1024],
            cursor: 0,
        }
    }

    pub fn alloc(&mut self, size: usize) -> Result<&mut [u8], String> {
        let end = self.cursor + size;

        if end > self.buffer.len() {
            return Err("Out of memory".to_string());
        }

        let allocation = &mut self.buffer[self.cursor..end];

        self.cursor = end;

        Ok(allocation)
    }
}
