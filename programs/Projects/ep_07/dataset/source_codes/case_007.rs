use anchor_lang::prelude::*;
use solana_program::program::invoke;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_007 {
    use super::*;
    pub fn execute_call(ctx: Context<DirectInvoke007>, lamports: u64) -> ProgramResult {
        // Prepare instruction for system program transfer
        let ix = solana_program::system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.target.key(),
            lamports,
        );
        // Arbitrary CPI without checking authority of program
        invoke(&ix, &[ctx.accounts.payer.to_account_info(), ctx.accounts.target.to_account_info()])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DirectInvoke007<'info> {
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub target: AccountInfo<'info>,
    /// CHECK: target program unchecked
    pub unchecked_prog: UncheckedAccount<'info>,
}