pub const BYTE_LEN: u8 = 8;
pub const MASK: u8 = BYTE_LEN - 1;

pub struct DynamicBitSet {
    content: Vec<u8>,
    bit_count: usize,
}

#[inline]
fn index(pos: usize) -> usize {
    pos / BYTE_LEN as usize
}
#[inline]
fn offset(pos: usize) -> u8 {
    (pos & MASK as usize) as u8
}
#[inline]
fn bit_mask(pos: usize) -> u8 {
    1 << offset(pos)
}

impl DynamicBitSet {
    pub fn new() -> Self {
        DynamicBitSet {
            content: vec![],
            bit_count: 0,
        }
    }

    pub fn push(&mut self, bit: bool) {
        let index = index(self.bit_count);
        if self.content.get(index).is_none() {
            self.content.insert(index, 0);
        }
        if bit {
            let offset = offset(self.bit_count);
            let cur = &mut self.content[index];
            *cur |= 1 << offset
        }
        self.bit_count += 1;
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.bit_count = 0;
    }

    pub fn size(&self) -> usize {
        self.bit_count
    }

    pub fn bit(&self, pos: usize) -> bool {
        assert!(pos < self.bit_count);
        let index = index(pos);
        let bit_mask = bit_mask(pos);
        (self.content[index] & bit_mask) != 0
    }

    /// find next bit start from pos
    pub fn find_next(&self, pos: usize) -> Option<usize> {
        if pos >= self.bit_count - 1 || self.bit_count == 0 {
            return None;
        }

        let pos = pos + 1; // for "next"
        let index = index(pos);
        let offset = offset(pos);
        let fore = self.content[index] >> offset;
        if fore > 0 {
            // current byte find bit
            let mut mask = 1;
            let mut count = 0;
            while (fore & mask) == 0 {
                mask <<= 1;
                count += 1;
            }
            Some(pos + count)
        } else {
            let len = BYTE_LEN as usize;
            self.find_next(index * len + (len - 1))
        }
    }
}

impl Into<Vec<u8>> for DynamicBitSet {
    fn into(self) -> Vec<u8> {
        self.content
    }
}

impl From<Vec<u8>> for DynamicBitSet {
    fn from(v: Vec<u8>) -> Self {
        let len = v.len();
        DynamicBitSet {
            content: v,
            bit_count: len * BYTE_LEN as usize,
        }
    }
}

impl From<&[u8]> for DynamicBitSet {
    fn from(v: &[u8]) -> Self {
        DynamicBitSet::from(v.to_vec())
    }
}
