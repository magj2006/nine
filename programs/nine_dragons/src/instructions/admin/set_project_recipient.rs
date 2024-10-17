use anchor_lang::prelude::*;

use crate::{error::NineDragonsError, states::Project};

pub fn set_recipient(ctx: Context<NewRecipient>, _name: String) -> Result<()> {
    let config = &mut ctx.accounts.project;

    config.set_recipient(ctx.accounts.new_recipient.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct NewRecipient<'info> {
    #[account(
        mut,
        constraint = owner.key() == project.owner @ NineDragonsError::NotProjectOwner
    )]
    owner: Signer<'info>,

    #[account(
        mut,
        seeds = [Project::PROJECT_SEED_PREFIX, original_owner.key().as_ref(), name.as_bytes()],
        bump
    )]
    project: Account<'info, Project>,

    new_recipient: SystemAccount<'info>,

    /// the account which is used to create config account
    original_owner: SystemAccount<'info>,
}
