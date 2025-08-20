use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_024 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_024(ctx: Context<CloseStruct024>) -> ProgramResult {
        let mut acc_024_lam = ctx.accounts.acc_024.to_account_info().lamports.borrow_mut();
        let mut dest_024_lam = ctx.accounts.dest_024.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_024_lam.checked_add(*acc_024_lam).unwrap_or_default();
        *dest_024_lam = new_dst;
        *acc_024_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_024_lam, *dest_024_lam);
        let new = *acc_024_lam + *dest_024_lam;
        *dest_024_lam = new;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_024.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_024.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct024<'info> {
    #[account(mut)]
    pub acc_024: AccountInfo<'info>,
    #[account(mut)]
    pub dest_024: AccountInfo<'info>,
}
