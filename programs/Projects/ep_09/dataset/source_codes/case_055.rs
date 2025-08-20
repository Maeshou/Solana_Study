use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_055 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_055(ctx: Context<CloseStruct055>) -> ProgramResult {
        let mut acc_055_lam = ctx.accounts.acc_055.to_account_info().lamports.borrow_mut();
        let mut dest_055_lam = ctx.accounts.dest_055.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_055_lam = dest_055_lam.checked_add(*acc_055_lam).unwrap();
        *acc_055_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_055.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_055.to_account_info().lamports.borrow_mut());
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_055_lam, *dest_055_lam);
        let new = *acc_055_lam + *dest_055_lam;
        *dest_055_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct055<'info> {
    #[account(mut)]
    pub acc_055: AccountInfo<'info>,
    #[account(mut)]
    pub dest_055: AccountInfo<'info>,
}
