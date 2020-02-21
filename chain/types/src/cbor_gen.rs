use byteorder::{BigEndian, WriteBytesExt};

pub const MajUnsignedInt: u8 = 0;
pub const MajNegativeInt: u8 = 1;
pub const MajByteString: u8 = 2;
pub const MajTextString: u8 = 3;
pub const MajArray: u8 = 4;
pub const MajMap: u8 = 5;
pub const MajTag: u8 = 6;
pub const MajOther: u8 = 7;

pub fn cbor_encode_major_type(t: u8, l: u64) -> Vec<u8> {
    let mut b = Vec::with_capacity(9);

    if l < 24 {
        b.push((t << 5) | l as u8);
    } else if l < (1 << 8) {
        b.push((t << 5) | 24);
        b.push(l as u8);
    } else if l < (1 << 16) {
        b.push((t << 5) | 25);
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(l as u16).unwrap();
        b.extend_from_slice(&wtr);
    } else if l < (1 << 32) {
        b.push((t << 5) | 26);
        let mut wtr = vec![];
        wtr.write_u32::<BigEndian>(l as u32).unwrap();
        b.extend_from_slice(&wtr);
    } else {
        b.push((t << 5) | 27);
        let mut wtr = vec![];
        wtr.write_u64::<BigEndian>(l).unwrap();
        b.extend_from_slice(&wtr);
    }

    b
}
