use std::ops::{Deref, DerefMut};

const HEADER: [u8; 8] = [0u8; 8];  
const HEADER_LEN: usize = HEADER.len();


type InnerBuf<const T: usize> = [u8; T];  

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct WriteBuf<const T: usize>(InnerBuf<T>);

impl<const T: usize> Deref for WriteBuf<T> {
    type Target = InnerBuf<T>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl<T> DerefMut for WriteBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl<const T: usize> WriteBuf<T> {
    pub fn new(size: usize) -> Self {
        todo!();
    } 

    pub fn append_head(buf: &[u8]) {
        let blen = buf.len();
        let header = blen.to_be_bytes();
        todo!();
    }
}
