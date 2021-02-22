use std::ops;

pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);

    fn read_range<R: ops::RangeBounds<u16>>(&mut self, range: R) -> Vec<u8> {
        let start = match range.start_bound() {
            ops::Bound::Included(address) => *address,
            ops::Bound::Excluded(address) => *address - 1,
            ops::Bound::Unbounded => u16::MIN,
        };
        let end = match range.end_bound() {
            ops::Bound::Included(address) => *address,
            ops::Bound::Excluded(address) => *address - 1,
            ops::Bound::Unbounded => u16::MAX,
        };
        if start > end {
            return vec![];
        }
        let length = end as usize - start as usize;
        let mut v = Vec::with_capacity(length);
        for address in start..=end {
            v.push(self.read(address));
        }
        v
    }
}
