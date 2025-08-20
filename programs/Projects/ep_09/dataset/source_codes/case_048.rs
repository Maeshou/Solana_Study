use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_048 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_048(ctx: Context<CloseStruct048>) -> ProgramResult {
        let mut acc_048_lam = ctx.accounts.acc_048.to_account_info().lamports.borrow_mut();
        let mut dest_048_lam = ctx.accounts.dest_048.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_048_lam = dest_048_lam.checked_add(*acc_048_lam).unwrap();
        *acc_048_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_048_lam;
        let src_val = *acc_048_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_048_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_048_lam, *dest_048_lam);
        let new = *acc_048_lam + *dest_048_lam;
        *dest_048_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct048<'info> {
    #[account(mut)]
    pub acc_048: AccountInfo<'info>,
    #[account(mut)]
    pub dest_048: AccountInfo<'info>,
}
