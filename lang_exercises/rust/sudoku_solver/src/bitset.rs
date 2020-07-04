use std::fmt;

#[derive(Copy, Clone)]
pub struct BitSet {
    pub storage : u16
}

impl BitSet {
    pub fn is_set(&self, idx : u8) -> bool {
        self.storage & (1 << idx) != 0
    }

    pub fn set(&mut self, idx : u8) {
        self.storage |= 1 << idx;
    }

    pub fn unset(&mut self, idx : u8) {
        self.storage &= !(1 << idx);
    }

    pub fn count(&self) -> usize{
        let mut count : usize = 0;
        for i in 0..9 {
            if self.is_set(i) {
                count += 1;
            }
        }
        count
    }
}

impl BitSet {
    pub fn full() -> BitSet {
        BitSet { storage : 0x1ff}
    }

    pub fn empty() -> BitSet {
        BitSet { storage : 0 }
    }
}

impl fmt::Octal for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:0>3o}", self.storage).replace("0", "_"))
    }
}