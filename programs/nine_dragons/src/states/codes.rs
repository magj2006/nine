use anchor_lang::prelude::*;
use std::mem;
use bytemuck::{ Pod, Zeroable };


#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct CodeList {
    pub codes: [u8; 8*4501],
}

impl CodeList {
    pub fn new_len(&self, len: usize) -> usize {
        let old_size = 8 + self.codes.len() * mem::size_of::<[u8; 8]>();
        let new_size = old_size + len * mem::size_of::<[u8; 8]>();
        if self.codes.len() > 10 * 1024 {
            new_size
        } else{
            old_size
        }
    }
}
