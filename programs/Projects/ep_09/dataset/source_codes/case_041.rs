use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_041 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_041(ctx: Context<CloseStruct041>) -> ProgramResult {
        let mut acc_041_lam = ctx.accounts.acc_041.to_account_info().lamports.borrow_mut();
        let mut dest_041_lam = ctx.accounts.dest_041.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_041_lam.checked_add(*acc_041_lam).unwrap_or_default();
        *dest_041_lam = new_dst;
        *acc_041_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_041_lam;
        let src_val = *acc_041_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_041_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_041.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_041.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct041<'info> {
    #[account(mut)]
    pub acc_041: AccountInfo<'info>,
    #[account(mut)]
    pub dest_041: AccountInfo<'info>,
}
