use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::types::{CollectionDetails, DataV2},
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, MintTo, Token},
    token_interface::{Mint, TokenAccount},
};

use crate::error::NineDragonsError;
use crate::states::{CreateCollectionParam, Project};

pub fn create_collection(
    ctx: Context<CreateCollection>,
    param: CreateCollectionParam,
) -> Result<()> {
    param.validate()?;

    let recipient = ctx.accounts.recipient.key();
    let project = &mut ctx.accounts.project;

    require_keys_eq!(recipient, project.recipient);

    require_eq!(project.nonce, 0, NineDragonsError::CreateCollectionFirst);

    project.collection_nft = Some(ctx.accounts.mint_account.key());

    let signer = &[Project::AUTHORITY_SEED_PREFIX, &[ctx.bumps.authority][..]];

    msg!("Minting Token");
    // Cross Program Invocation (CPI)
    // Invoking the mint_to instruction on the token program
    mint_to(
        CpiContext::new_with_signer(
            // CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.collection_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
            &[signer],
        ),
        1,
    )?;

    let collection_details = Some(CollectionDetails::V2 { padding: [0; 8] });
    // let collection_details = None;

    msg!("Creating metadata account");
    // Cross Program Invocation (CPI)
    // Invoking the create_metadata_account_v3 instruction on the token metadata program
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[signer],
        ),
        DataV2 {
            name: param.name,
            symbol: param.symbol,
            uri: param.uri,
            seller_fee_basis_points: project.seller_fee_basis_points,
            creators: None,
            collection: None,
            uses: None,
        },
        false,              // Is mutable
        true,               // Update authority is signer
        collection_details, // Collection details
    )?;

    msg!("Creating master edition account");
    // Cross Program Invocation (CPI)
    // Invoking the create_master_edition_v3 instruction on the token metadata program
    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.edition_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[signer],
        ),
        Some(0), // Max Supply
    )?;

    project.nonce += 1;

    msg!("Collection minted successfully. nonce: {}", project.nonce);

    Ok(())
}

#[derive(Accounts)]
#[instruction(param: CreateCollectionParam)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    // // // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = payer,
        seeds = ["mint".as_bytes(), project.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub mint_account: Box<InterfaceAccount<'info, Mint>>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
    )]
    pub project: Box<Account<'info, Project>>,

    /// CHECK: PDA account
    #[account(
        mut,
        seeds = [Project::AUTHORITY_SEED_PREFIX],
        bump
    )]
    authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
