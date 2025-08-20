use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_008 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_008(ctx: Context<CloseStruct008>) -> ProgramResult {
        let mut acc_008_lam = ctx.accounts.acc_008.to_account_info().lamports.borrow_mut();
        let mut dest_008_lam = ctx.accounts.dest_008.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_008.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_008.to_account_info().lamports.borrow_mut());
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_008_lam;
        let src_val = *acc_008_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_008_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_008_lam.checked_add(*acc_008_lam).unwrap_or_default();
        *dest_008_lam = new_dst;
        *acc_008_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct008<'info> {
    #[account(mut)]
    pub acc_008: AccountInfo<'info>,
    #[account(mut)]
    pub dest_008: AccountInfo<'info>,
}
