// Scenario 085: 複数シグ入金（Multisig Deposit）
use anchor_lang::prelude::*;
declare_id!("Fake22222222222222222222222222222222");

#[program]
pub mod case_085 {
    pub fn execute_tx(ctx: Context<Case_085Ctx>, data: u64) -> Result<()> {
        let user_acc = &mut ctx.accounts.user_account;
        let recipient_acc = &mut ctx.accounts.recipient_account;
        let from_balance = **user_acc.to_account_info().lamports.borrow();
        **user_acc.to_account_info().try_borrow_mut_lamports()? = from_balance.saturating_sub(data);
        **recipient_acc.to_account_info().try_borrow_mut_lamports()? += data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case_085Ctx<'info> {
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    #[account(mut)]
    pub recipient_account: AccountInfo<'info>,
    pub authority: Signer<'info>,
}
