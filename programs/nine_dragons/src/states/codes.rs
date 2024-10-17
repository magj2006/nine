use anchor_lang::prelude::*;
use std::mem;

#[account]
#[derive(InitSpace)]
pub struct CodeList {
    #[max_len(0)]
    pub codes: Vec<[u8; 8]>,
}

impl CodeList {
    pub fn new_len(&self, len: usize) -> usize {
        8 + 4 + (self.codes.len() + len) * mem::size_of::<[u8; 8]>()
    }
}
