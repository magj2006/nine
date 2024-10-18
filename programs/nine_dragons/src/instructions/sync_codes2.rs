use anchor_lang::prelude::*;
use crate::states::{CodeList2, Project};
use crate::error::*;

const CODES_MAX_LEN: usize = 1500;

pub fn sync_codes2(ctx: Context<SyncCodes2>, param: SyncCodesParam2) -> Result<()> {

    param.require_len()?;

    let code_account = &mut ctx.accounts.codes2;

    require_gte!(CODES_MAX_LEN, code_account.codes.len());

    code_account.codes.extend(&param.input_codes);

    msg!("Successful to sync code");

    Ok(())
}


#[derive(Accounts)]
#[instruction(param: SyncCodesParam2)]
pub struct SyncCodes2<'info> {
    #[account(mut,
    )]
    pub operator: Signer<'info>,

    #[account(
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
        has_one = operator @ NineDragonsError::NotAllowedOperator,
        has_one = codes2 @ NineDragonsError::InvalidCode
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        realloc = codes2.new_len(param.len()),
        realloc::payer = operator,
        realloc::zero = false
    )]
    pub codes2: Account<'info, CodeList2>,

    /// CHECK: only read, the account which is used to create config account
    pub original_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SyncCodesParam2 {
    input_codes: Vec<[u8; 8]>
}

impl SyncCodesParam2 {
    pub fn len(&self) -> usize {
        self.input_codes.len()
    }
}

impl SyncCodesParam2 {
    pub fn require_len(&self) -> Result<()>  {
        require_gte!(300, self.input_codes.len(), NineDragonsError::MoreThanLimit);

        Ok(())
    }
}