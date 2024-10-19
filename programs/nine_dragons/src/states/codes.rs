use anchor_lang::prelude::*;
use std::mem;
use bytemuck::{ Pod, Zeroable };


#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct CodeList {
    pub codes: [u8; 8 * 100],
    pub current_index: u32,
}
