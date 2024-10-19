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

    let code_list = &mut ctx.accounts.codes.load_mut()?;

    let current_size = code_list.current_size as usize;
    let end_size = current_size + 100 * 8;

    if end_size > param.new_size as usize {
        return Err(NineDragonsError::AccountDataTooSmall.into());
    }

    code_list.codes[current_size..end_size].copy_from_slice(&param.input_codes);
    code_list.current_size = end_size as u32;

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
        realloc = 8 + param.new_size as usize * 8,
        seeds = [Project::CODES1_SEED_PREFIX],
        bump,
        realloc::payer = operator,
        realloc::zero = false,
    )]
    pub codes: AccountLoader<'info, CodeList>,

    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SyncCodesParam {
    input_codes: [u8; 8 * 100],
    new_size: u32,
}