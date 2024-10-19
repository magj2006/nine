use anchor_lang::prelude::*;

use crate::error::*;
use crate::states::{CodeList, Project};

const CODES_MAX_LEN: usize = 1500;

pub fn sync_codes(ctx: Context<SyncCodes1>, param: SyncCodesParam1) -> Result<()> {
    param.require_len()?;

    let code_account = &mut ctx.accounts.codes;

    require_gte!(CODES_MAX_LEN, code_account.codes.len());

    let project = &ctx.accounts.project;
    assert!(project.codes1 == code_account.key() ||
        project.codes2 == code_account.key() ||
        project.codes3 == code_account.key() ||
        project.codes4 == code_account.key() ||
        project.codes5 == code_account.key());

    code_account.codes.extend(&param.input_codes);

    msg!("Successful to sync code");

    Ok(())
}


#[derive(Accounts)]
#[instruction(param: SyncCodesParam1)]
pub struct SyncCodes1<'info> {
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
        realloc = codes.new_len(param.len()),
        realloc::payer = operator,
        realloc::zero = false
    )]
    pub codes: Box<Account<'info, CodeList>>,

    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SyncCodesParam1 {
    input_codes: Vec<[u8; 8]>,
}

impl SyncCodesParam1 {
    pub fn len(&self) -> usize {
        self.input_codes.len()
    }
}

impl SyncCodesParam1 {
    pub fn require_len(&self) -> Result<()> {
        require_gte!(300, self.input_codes.len(), NineDragonsError::MoreThanLimit);

        Ok(())
    }
}