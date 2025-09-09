//use std::ops::{Deref, DerefMut};

pub(crate) type Header = [u8; 8];
pub(crate) const HEADER: Header = [0u8; 8];  
pub(crate) const HEADER_LEN: usize = HEADER.len();

pub(crate) fn header(data_buf: &[u8]) -> Header {
    let blen = data_buf.as_ref().len();

    #[cfg(target_endian="big")]
    return blen.to_be_bytes();

    #[cfg(target_endian="little")]
    return blen.to_le_bytes();
}

pub(crate) fn header_len(header: &Header) -> u64 {
    #[cfg(target_endian = "big")]
    return u64::from_be_bytes(header.clone());

    #[cfg(target_endian = "little")]
    return u64::from_be_bytes(header.clone());
}
