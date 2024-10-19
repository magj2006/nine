use anchor_lang::prelude::*;
use crate::states::{CodeList, Project};
use crate::error::*;


pub fn sync_codes(ctx: Context<SyncCodes>, param: SyncCodesParam) -> Result<()> {

    let code_account = &mut ctx.accounts.codes;

    let project = &ctx.accounts.project;

    // assert!(project.codes == code_account.key() ||
    // project.codes2 == code_account.key() ||
    // project.codes3 == code_account.key());
    require_keys_eq!(project.codes, code_account.key());

    ctx.accounts
        .codes.load_mut()?.codes[param.start as usize..(param.start + 100 * 8) as usize].copy_from_slice(&param.input_codes);

    // code_account.codes.extend(&param.input_codes);

    msg!("Successful to sync code");

    Ok(())
}


#[derive(Accounts)]
#[instruction(param: SyncCodesParam)]
pub struct SyncCodes<'info> {
    #[account(mut,
    )]
    pub operator: Signer<'info>,

    #[account(
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
        has_one = operator @ NineDragonsError::NotAllowedOperator,
    )]
    pub project: Box<Account<'info, Project>>,

    #[account(
        mut,
        realloc = 8 * 100,
        realloc::payer = operator,
        realloc::zero = false
    )]
    pub codes: AccountLoader<'info, CodeList>,

    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SyncCodesParam {
    input_codes: [u8; 8 * 100],
    start: u32,
}