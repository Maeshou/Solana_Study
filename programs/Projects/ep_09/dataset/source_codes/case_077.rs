use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_077 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_077(ctx: Context<CloseStruct077>) -> ProgramResult {
        let mut acc_077_lam = ctx.accounts.acc_077.to_account_info().lamports.borrow_mut();
        let mut dest_077_lam = ctx.accounts.dest_077.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_077_lam = dest_077_lam.checked_add(acc_077_lam.checked_add(0).unwrap()).unwrap();
        *acc_077_lam = acc_077_lam.checked_sub(acc_077_lam).unwrap();
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_077_lam;
        let src_val = *acc_077_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_077_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_077.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_077.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct077<'info> {
    #[account(mut)]
    pub acc_077: AccountInfo<'info>,
    #[account(mut)]
    pub dest_077: AccountInfo<'info>,
}
