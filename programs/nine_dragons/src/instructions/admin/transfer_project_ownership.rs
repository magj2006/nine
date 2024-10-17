use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::associated_token::{create as create_ata, AssociatedToken};
use anchor_spl::token_interface::{TokenAccount, TokenInterface, transfer_checked, TransferChecked};
use anchor_spl::token_interface::Mint;

use crate::{error::NineDragonsError, states::Project};

pub fn transfer_project_ownership(ctx: Context<ChangeProjectOwnership>, _project_name: String) -> Result<()> {
    let project = &mut ctx.accounts.project;

    if let Some(collection_nft) = project.collection_nft {
        if let Some(collection) = &ctx.accounts.collection {
            require_keys_eq!(collection_nft, collection.key(), NineDragonsError::InvalidCollection);

            create_ata(
                CpiContext::new(
                    ctx.accounts.associated_token_program.to_account_info(),
                    associated_token::Create {
                        payer: ctx.accounts.owner.to_account_info(),
                        associated_token: ctx.accounts.new_collection_token_account.to_account_info(),
                        authority: ctx.accounts.new_owner.to_account_info(),
                        mint: collection.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
                    },
                ),
            )?;

            transfer_checked(
                CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.collection_token_account.to_account_info(),
                    mint: collection.to_account_info(),
                    to: ctx.accounts.new_collection_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }),
                1,
                0
            )?;

        } else {
            return Err(NineDragonsError::EmptyCollection.into())
        }
    }

    project.change_ownership(ctx.accounts.new_owner.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct ChangeProjectOwnership<'info> {
    #[account(
        mut,
        constraint = owner.key() == project.owner @ NineDragonsError::NotProjectOwner
    )]
    owner: Signer<'info>,

    #[account(
        mut,
        seeds = [Project::PROJECT_SEED_PREFIX, owner.key().as_ref(), project_name.as_bytes()],
        bump
    )]
    project: Account<'info, Project>,
    new_owner: SystemAccount<'info>,

    #[account(mut)]
    collection_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// UNCHECK: may be no need it
    pub new_collection_token_account: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = project.update_authority == authority.key()
    )]
    authority: SystemAccount<'info>,

    /// the account which is used to create config account
    original_owner: SystemAccount<'info>,
    collection: Option<InterfaceAccount<'info, Mint>>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>
}
