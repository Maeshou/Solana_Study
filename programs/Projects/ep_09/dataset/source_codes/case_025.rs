use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_025 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_025(ctx: Context<CloseStruct025>) -> ProgramResult {
        let mut acc_025_lam = ctx.accounts.acc_025.to_account_info().lamports.borrow_mut();
        let mut dest_025_lam = ctx.accounts.dest_025.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_025_lam.checked_add(*acc_025_lam).unwrap_or_default();
        *dest_025_lam = new_dst;
        *acc_025_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_025_lam + *dest_025_lam;
        *dest_025_lam = sum;
        *acc_025_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_025.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_025.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct025<'info> {
    #[account(mut)]
    pub acc_025: AccountInfo<'info>,
    #[account(mut)]
    pub dest_025: AccountInfo<'info>,
}
