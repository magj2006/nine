use anchor_lang::prelude::*;
use crate::states::{CodeList3, Project};
use crate::error::*;

const CODES_MAX_LEN: usize = 1500;

pub fn sync_codes3(ctx: Context<SyncCodes3>, param: SyncCodesParam3) -> Result<()> {

    param.require_len()?;

    let code_account = &mut ctx.accounts.codes3;

    require_gte!(CODES_MAX_LEN, code_account.codes.len());

    code_account.codes.extend(&param.input_codes);

    msg!("Successful to sync code");

    Ok(())
}


#[derive(Accounts)]
#[instruction(param: SyncCodesParam3)]
pub struct SyncCodes3<'info> {
    #[account(mut,
    )]
    pub operator: Signer<'info>,

    #[account(
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
        has_one = operator @ NineDragonsError::NotAllowedOperator,
        has_one = codes3 @ NineDragonsError::InvalidCode
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        realloc = codes3.new_len(param.len()),
        realloc::payer = operator,
        realloc::zero = false
    )]
    pub codes3: Account<'info, CodeList3>,

    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SyncCodesParam3 {
    input_codes: Vec<[u8; 8]>
}

impl SyncCodesParam3 {
    pub fn len(&self) -> usize {
        self.input_codes.len()
    }
}

impl SyncCodesParam3 {
    pub fn require_len(&self) -> Result<()>  {
        require_gte!(300, self.input_codes.len(), NineDragonsError::MoreThanLimit);

        Ok(())
    }
}