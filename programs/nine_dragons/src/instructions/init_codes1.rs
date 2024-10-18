use anchor_lang::prelude::*;
use crate::states::{CodeList, Project};

pub fn init_codes1(ctx: Context<InitCodes1>) -> Result<()> {

    let project = &mut ctx.accounts.project;

    project.codes1 = ctx.accounts.codes.key();

    msg!("Init codes");

    Ok(())
}


#[derive(Accounts)]
pub struct InitCodes1<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        mut,
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
    )]
    project: Box<Account<'info, Project>>,

    #[account(
        init,
        payer = owner,
        seeds = [Project::CODES1_SEED_PREFIX],
        bump,
        space = 8 + CodeList::INIT_SPACE
    )]
    codes: Box<Account<'info, CodeList>>,
    system_program: Program<'info, System>
}