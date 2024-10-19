use anchor_lang::prelude::*;
use crate::states::{CodeList, Project};

pub fn init_codes(ctx: Context<InitCodes1>) -> Result<()> {

    let project = &mut ctx.accounts.project;

    project.codes = ctx.accounts.codes.key();

    let code_list = &mut ctx.accounts.codes.load_init()?;

    code_list.codes = [0u8; 8 * 100];
    code_list.current_size = 0;

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
        space= 8 + 8 * 100 as usize
    )]
    codes: AccountLoader<'info, CodeList>,
    system_program: Program<'info, System>
}