use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_088 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_088(ctx: Context<CloseStruct088>) -> ProgramResult {
        let mut acc_088_lam = ctx.accounts.acc_088.to_account_info().lamports.borrow_mut();
        let mut dest_088_lam = ctx.accounts.dest_088.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_088_lam.checked_add(*acc_088_lam).unwrap_or_default();
        *dest_088_lam = new_dst;
        *acc_088_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_088.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_088.to_account_info().lamports.borrow_mut());
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_088_lam;
        let src_val = *acc_088_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_088_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct088<'info> {
    #[account(mut)]
    pub acc_088: AccountInfo<'info>,
    #[account(mut)]
    pub dest_088: AccountInfo<'info>,
}
