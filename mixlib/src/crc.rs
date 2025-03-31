use std::hash::Hash;
use std::path::Path;

#[derive(PartialEq, Eq)]
pub struct CRC(pub u32);

impl Hash for CRC {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

impl CRC {
    /// Was Calculate_CRC in C++ code.
    pub fn calculate(filename: &Path) -> Option<Self> {
        match filename.to_str() {
            None => None,
            Some(filename) => Some(CRC(Self::calc_u32(filename.as_bytes()))),
        }
    }

    pub(crate) const fn calc_u32_from_str(bytes: &str) -> u32 {
        Self::calc_u32(bytes.as_bytes())
    }

    /// Implements CRC implementation known to work for Tiberium Dawn.
    /// Might now work with Mixfiles of other games.
    ///
    /// Implementation uses as much unconditional code as possible
    /// in the hope for this all to get turned into SIMD operations
    /// by the compiler.
    pub(crate) const fn calc_u32(bytes: &[u8]) -> u32 {
        let mut id: u32 = 0;
        let l = bytes.len();
        let padded_len = l + (4 - l % 4) % 4;

        let mut chunk = 0;
        let chunk_count = padded_len / 4;
        while chunk < chunk_count {
            let mut a: u32 = 0;

            // Fixed size loops should be inlined
            let mut j = 0;
            const J_COUNT: usize = 4;
            while j < J_COUNT {
                a >>= 8;

                let i = chunk * 4 + j;
                // Both (i < l) expressions switch from `true` to `false` when out of bounds access
                // will be attempted. Converting them to unsigned gives us `1` and `0` respectively.
                // For in bounds this means this does not modify the index used and value returned.
                // For out of bounds this effectively resets the index used to `0` and also returns
                // the value `0`.
                // The value `0` in turn the makes the a operation not change the hash.
                let b = ((i < l) as u8) * bytes[i * ((i < l) as usize)];
                a += (b as u32) << 24;

                j += 1;
            }
            id = (id << 1 | id >> 31).wrapping_add(a);

            chunk += 1;
        }

        id
    }
}
