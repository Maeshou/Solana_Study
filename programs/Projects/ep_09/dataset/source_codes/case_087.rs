use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_087 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_087(ctx: Context<CloseStruct087>) -> ProgramResult {
        let mut acc_087_lam = ctx.accounts.acc_087.to_account_info().lamports.borrow_mut();
        let mut dest_087_lam = ctx.accounts.dest_087.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_087_lam, *dest_087_lam);
        let new = *acc_087_lam + *dest_087_lam;
        *dest_087_lam = new;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_087.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_087.to_account_info().lamports.borrow_mut());
    

        // Snippet: checked_add with unwrap
        *dest_087_lam = dest_087_lam.checked_add(*acc_087_lam).unwrap();
        *acc_087_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct087<'info> {
    #[account(mut)]
    pub acc_087: AccountInfo<'info>,
    #[account(mut)]
    pub dest_087: AccountInfo<'info>,
}
