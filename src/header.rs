pub(crate) type Header = [u8; 8];
pub(crate) const HEADER: Header = [0u8; 8];  
pub(crate) const HEADER_LEN: usize = HEADER.len();

/// the header does not include the length of itself
pub(crate) fn header(data_buf: &[u8]) -> Header {
    let blen = data_buf.as_ref().len();

    #[cfg(target_endian = "big")]
    return blen.to_be_bytes();

    #[cfg(target_endian = "little")]
    return blen.to_le_bytes();
}

/// the header does not include the length of itself
pub(crate) fn header_len(header: &Header) -> usize {
    #[cfg(target_endian = "big")]
    return u64::from_be_bytes(header.clone());

    #[cfg(target_endian = "little")]
    return (u64::from_be_bytes(header.clone())) as usize;
}
