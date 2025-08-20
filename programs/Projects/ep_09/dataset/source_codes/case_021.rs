use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_021 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_021(ctx: Context<CloseStruct021>) -> ProgramResult {
        let mut acc_021_lam = ctx.accounts.acc_021.to_account_info().lamports.borrow_mut();
        let mut dest_021_lam = ctx.accounts.dest_021.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_021_lam, *dest_021_lam);
        let new = *acc_021_lam + *dest_021_lam;
        *dest_021_lam = new;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_021_lam.checked_add(*acc_021_lam).unwrap_or_default();
        *dest_021_lam = new_dst;
        *acc_021_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_021_lam = dest_021_lam.checked_add(*acc_021_lam).unwrap();
        *acc_021_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct021<'info> {
    #[account(mut)]
    pub acc_021: AccountInfo<'info>,
    #[account(mut)]
    pub dest_021: AccountInfo<'info>,
}
