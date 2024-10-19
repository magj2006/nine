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

    // require_gte!(param.len, code_list.len, );

    let current_index = code_list.current_index as usize;
    let end_index = current_index + param.len as usize * 8;

    if end_index > param.len as usize * 8 + 8 + 4 {
        return Err(NineDragonsError::AccountDataTooSmall.into())
    }

    msg!("current index: {current_index}, end index: {end_index}");

    code_list.codes[current_index ..end_index].copy_from_slice(&param.input_codes);
    code_list.current_index = end_index as u32;
    // code_list.len = param.len;

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
        realloc = 8 + 4 + param.len as usize * 8,
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
    // input_codes: [u8; 8 * 100],
    input_codes: Vec<u8>,
    len: u32,
}