use anchor_lang::prelude::*;

mod error;
mod instructions;
mod states;

declare_id!("BMm21yWi9vMxTnxFPUtweNdW53wXbRktBPGbcwyq4CxX");

use instructions::*;

use crate::states::{CreateCollectionParam, CreateNFTParam};

#[program]
pub mod nine_dragons {
    use super::*;

    pub fn init_project(
        ctx: Context<InitProject>,
        price: u64,
        seller_fee_basis_points: u16,
        is_mutable: bool,
    ) -> Result<()> {
        instructions::init_project(ctx, price, seller_fee_basis_points, is_mutable)
    }

    pub fn set_recipient(ctx: Context<NewRecipient>, name: String) -> Result<()> {
        instructions::set_recipient(ctx, name)
    }

    pub fn set_price(ctx: Context<NewPrice>, name: String, price: u64) -> Result<()> {
        instructions::set_price(ctx, name, price)
    }

    pub fn accept_project_ownership(ctx: Context<AcceptProjectOwnership>, name: String) -> Result<()> {
        instructions::accept_project_ownership(ctx, name)
    }

    pub fn transfer_project_ownership(ctx: Context<ChangeProjectOwnership>, name: String) -> Result<()> {
        instructions::transfer_project_ownership(ctx, name)
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        param: CreateCollectionParam,
    ) -> Result<()> {
        instructions::create_collection(ctx, param)
    }

    pub fn create_nft(
        ctx: Context<CreateNFT>,
        param: CreateNFTParam,
    ) -> Result<()> {
        instructions::create_nft(ctx, param)
    }

    pub fn sync_codes1(ctx: Context<SyncCodes>, param: SyncCodesParam) -> Result<()> {
        instructions::sync_codes(ctx, param)
    }

    pub fn init_codes(ctx: Context<InitCodes>) -> Result<()> {
        instructions::init_codes(ctx)
    }

    // pub fn init_codes2(ctx: Context<InitCodes2>) -> Result<()> {
    //     instructions::init_codes2(ctx)
    // }
    //
    // pub fn init_codes3(ctx: Context<InitCodes3>) -> Result<()> {
    //     instructions::init_codes3(ctx)
    // }

}

