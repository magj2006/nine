use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata, mpl_token_metadata::types::{Collection, DataV2},
    },
    token::{mint_to, MintTo, Token},
    token_interface::{Mint, TokenAccount},
};
use anchor_spl::metadata::{verify_collection, verify_sized_collection_item, VerifySizedCollectionItem, VerifyCollection};

use crate::error::NineDragonsError;
use crate::states::{CodeList, CreateNFTParam, Project};

pub fn create_nft(
    ctx: Context<CreateNFT>,
    param: CreateNFTParam,
) -> Result<()> {
    param.validate()?;

    let recipient = ctx.accounts.recipient.key();
    let project = &mut ctx.accounts.project;
    require_keys_eq!(recipient, project.recipient);

    require_gt!(project.nonce, 0, NineDragonsError::CreateCollectionFirst);

    let code_list = &mut ctx.accounts.codes;
    require!(code_list.codes.iter().any(|code| code.eq(&param.code)), NineDragonsError::InvalidCode);

    require!(project.collection_nft.is_some(), NineDragonsError::EmptyCollection);
    require_keys_eq!(project.collection_nft.unwrap().key(), ctx.accounts.collection.key(), NineDragonsError::InvalidCollection);

    if project.price > 0 {
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                },
            ),
            project.price,
        )?;
    }

    let signer = &[
        Project::AUTHORITY_SEED_PREFIX,
        &[ctx.bumps.authority][..]
    ];

    msg!("Minting Token");
    // Cross Program Invocation (CPI)
    // Invoking the mint_to instruction on the token program
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.nft_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
            &[signer],
        ),
        1,
    )?;


    msg!("Collection: {}", ctx.accounts.collection.key());

    let collection = Collection {
        key: ctx.accounts.collection.key(),
        verified: false,
    };

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
            collection: Some(collection),
            uses: None,
        },
        false,              // Is mutable
        true,               // Update authority is signer
        None, // Collection details
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
        Some(1), // Max Supply
    )?;

    project.nonce += 1;

    msg!("NFT minted successfully. nonce: {}", project.nonce);

    // verify_collection(
    verify_sized_collection_item(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            VerifySizedCollectionItem {
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                collection_authority: ctx.accounts.authority.to_account_info(),
                collection_mint: ctx.accounts.collection.to_account_info(),
                collection_metadata: ctx.accounts.collection_metadata_account.to_account_info(),
                collection_master_edition: ctx.accounts.collection_edition_account.to_account_info(),
            },
            &[signer]),
        None)?;

    msg!("NFT is verified successfully");

    Ok(())
}

#[derive(Accounts)]
#[instruction(param: CreateNFTParam)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    edition_account: UncheckedAccount<'info>,

    // // // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = payer,
        seeds = ["mint".as_bytes(), param.code.as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    mint_account: Box<InterfaceAccount<'info, Mint>>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    nft_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
        has_one = codes @ NineDragonsError::InvalidCodesAccount
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
    collection: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut
    )]
    codes: Account<'info, CodeList>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    collection_metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        seeds = [b"metadata", token_metadata_program.key().as_ref(), collection.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    collection_edition_account: UncheckedAccount<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,
    /// CHECK: only read, the account which is used to create config account
    original_owner: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    token_metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

