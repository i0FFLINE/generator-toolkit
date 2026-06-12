// src/random/os_rng.rs
use getrandom::fill;

pub struct OsRng;

impl OsRng {
    pub fn new() -> Self {
        OsRng
    }

    /// Returns a random u32
    fn u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        fill(&mut buf).expect("Failed to get random bytes");
        u32::from_ne_bytes(buf)
    }

    /// Returns a random usize in [0, max) using rejection sampling
    pub fn gen_range(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        let limit = (u32::MAX / max as u32) * max as u32;
        loop {
            let r = self.u32();
            if r < limit {
                return (r % max as u32) as usize;
            }
        }
    }

    /// Randomly selects an element from a slice
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.gen_range(slice.len());
            Some(&slice[idx])
        }
    }
}
