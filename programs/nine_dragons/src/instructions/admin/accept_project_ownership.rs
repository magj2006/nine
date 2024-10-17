use anchor_lang::prelude::*;

use crate::{error::NineDragonsError, states::Project};

pub fn accept_project_ownership(ctx: Context<AcceptProjectOwnership>, _name: String) -> Result<()> {
    let config = &mut ctx.accounts.project;

    config.accept_ownership(ctx.accounts.new_owner.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AcceptProjectOwnership<'info> {
    #[account(
        mut,
        constraint = new_owner.key() == project.pending_owner.unwrap() @ NineDragonsError::NotNewProjectOwner
    )]
    pub new_owner: Signer<'info>,

    #[account(
        mut,
        seeds = [Project::PROJECT_SEED_PREFIX, original_owner.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,

    /// the account which is used to create config account
    pub original_owner: SystemAccount<'info>,
}
