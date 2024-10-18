use anchor_lang::prelude::*;
use std::mem;

#[account]
#[derive(InitSpace)]
pub struct CodeList1 {
    #[max_len(10)]
    pub codes: Vec<[u8; 8]>,
}

impl CodeList1 {
    pub fn new_len(&self, len: usize) -> usize {
        8 + 4 + (self.codes.len() + len) * mem::size_of::<[u8; 8]>()
    }
}

#[account]
#[derive(InitSpace)]
pub struct CodeList2 {
    #[max_len(10)]
    pub codes: Vec<[u8; 8]>,
}

impl CodeList2 {
    pub fn new_len(&self, len: usize) -> usize {
        8 + 4 + (self.codes.len() + len) * mem::size_of::<[u8; 8]>()
    }
}

#[account]
#[derive(InitSpace)]
pub struct CodeList3 {
    #[max_len(10)]
    pub codes: Vec<[u8; 8]>,
}

impl CodeList3 {
    pub fn new_len(&self, len: usize) -> usize {
        8 + 4 + (self.codes.len() + len) * mem::size_of::<[u8; 8]>()
    }
}