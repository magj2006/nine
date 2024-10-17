use anchor_lang::prelude::*;

use crate::{error::NineDragonsError, states::Project};

pub fn set_price(ctx: Context<NewPrice>, _name: String, price: u64) -> Result<()> {
    let config = &mut ctx.accounts.project;

    config.set_price(price);

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct NewPrice<'info> {
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

    /// the account which is used to create config account
    original_owner: SystemAccount<'info>,
}
