use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_007 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_007(ctx: Context<CloseStruct007>) -> ProgramResult {
        let mut acc_007_lam = ctx.accounts.acc_007.to_account_info().lamports.borrow_mut();
        let mut dest_007_lam = ctx.accounts.dest_007.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_007_lam;
        let src_val = *acc_007_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_007_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_007_lam, *dest_007_lam);
        let new = *acc_007_lam + *dest_007_lam;
        *dest_007_lam = new;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_007_lam.checked_add(*acc_007_lam).unwrap_or_default();
        *dest_007_lam = new_dst;
        *acc_007_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct007<'info> {
    #[account(mut)]
    pub acc_007: AccountInfo<'info>,
    #[account(mut)]
    pub dest_007: AccountInfo<'info>,
}
