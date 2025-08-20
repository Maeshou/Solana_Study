use anchor_lang::prelude::*;
use solana_program::program::invoke;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_035 {
    use super::*;
    pub fn post_memo(ctx: Context<MemoVuln035>, data: Vec<u8>) -> ProgramResult {
        let ix = spl_memo::build_memo(&data, &[]);
        // Arbitrary invocation without checking memo program id
        invoke(&ix, &[ctx.accounts.memo_prog.to_account_info()])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MemoVuln035<'info> {
    /// CHECK: memo program unchecked
    pub memo_prog: UncheckedAccount<'info>,
}