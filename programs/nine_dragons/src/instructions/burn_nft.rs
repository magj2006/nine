use anchor_lang::prelude::*;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::{burn, Burn, close_account, CloseAccount, Token};
use anchor_spl::token_interface::{Mint, TokenAccount};
use anchor_spl::metadata::{burn_nft as burn_nft_metadata, BurnNft as BurnNftMetadata};
use crate::states::Project;


pub fn burn_nft(ctx: Context<BurnNft>) -> Result<()> {

    let signer = &[
        Project::AUTHORITY_SEED_PREFIX,
        &[ctx.bumps.authority][..]
    ];

    // burn
    burn(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.nft.to_account_info(),
            from: ctx.accounts.from_token_account.to_account_info(),
            authority: ctx.accounts.from.to_account_info()
        },
        &[signer]),
         1)?;

    // close
    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft.to_account_info(),
            destination: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        },
        &[signer]))?;


    // delete metadata
    burn_nft_metadata(CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        BurnNftMetadata {
            metadata: ctx.accounts.metadata.to_account_info(),
            owner: ctx.accounts.from.to_account_info(),
            mint: ctx.accounts.nft.to_account_info(),
            token: ctx.accounts.from_token_account.to_account_info(),
            edition: ctx.accounts.edition.to_account_info(),
            spl_token: ctx.accounts.token_program.to_account_info()
        }, &[signer]), None)?;


    Ok(())
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    pub nft: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        token::mint = nft
    )]
    pub from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: metadata account
    #[account(
    mut,
    seeds = [b"metadata", token_metadata_program.key().as_ref(), nft.key().as_ref()],
    bump,
    seeds::program = token_metadata_program.key(),
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: edition account
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), nft.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition: UncheckedAccount<'info>,

    /// CHECK: PDA account
    #[account(
        mut,
        seeds = [Project::AUTHORITY_SEED_PREFIX],
        bump
    )]
    pub authority: UncheckedAccount<'info>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
}