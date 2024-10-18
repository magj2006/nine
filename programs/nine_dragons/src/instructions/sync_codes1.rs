use anchor_lang::prelude::*;
use crate::states::{CodeList1, Project};
use crate::error::*;

const CODES_MAX_LEN: usize = 1500;

pub fn sync_codes1(ctx: Context<SyncCodes1>, param: SyncCodesParam1) -> Result<()> {

    param.require_len()?;

    let code_account = &mut ctx.accounts.codes1;

    require_gte!(CODES_MAX_LEN, code_account.codes.len());

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
        has_one = codes1 @ NineDragonsError::InvalidCode
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        realloc = codes1.new_len(param.len()),
        realloc::payer = operator,
        realloc::zero = false
    )]
    pub codes1: Account<'info, CodeList1>,

    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SyncCodesParam1 {
    input_codes: Vec<[u8; 8]>
}

impl SyncCodesParam1 {
    pub fn len(&self) -> usize {
        self.input_codes.len()
    }
}

impl SyncCodesParam1 {
    pub fn require_len(&self) -> Result<()>  {
        require_gte!(300, self.input_codes.len(), NineDragonsError::MoreThanLimit);

        Ok(())
    }
}