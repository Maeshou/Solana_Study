use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_097 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_097(ctx: Context<CloseStruct097>) -> ProgramResult {
        let mut acc_097_lam = ctx.accounts.acc_097.to_account_info().lamports.borrow_mut();
        let mut dest_097_lam = ctx.accounts.dest_097.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_097.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_097.to_account_info().lamports.borrow_mut());
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_097_lam.checked_add(*acc_097_lam).unwrap_or_default();
        *dest_097_lam = new_dst;
        *acc_097_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_097_lam;
        let src_val = *acc_097_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_097_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct097<'info> {
    #[account(mut)]
    pub acc_097: AccountInfo<'info>,
    #[account(mut)]
    pub dest_097: AccountInfo<'info>,
}
