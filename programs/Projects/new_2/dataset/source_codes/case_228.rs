use anchor_lang::prelude::*;

declare_id!("VulnEx26000000000000000000000000000000000026");

#[program]
pub mod loan_repayment {
    pub fn repay(ctx: Context<Ctx6>, amt: u64) -> Result<()> {
        // repay_log は未検証
        ctx.accounts.repay_log.data.borrow_mut().extend_from_slice(&amt.to_le_bytes());
        // loan_account は has_one で borrower 検証済み
        let l = &mut ctx.accounts.loan_account;
        l.outstanding = l.outstanding.saturating_sub(amt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx6<'info> {
    /// CHECK: 返済ログ、所有者検証なし
    #[account(mut)]
    pub repay_log: AccountInfo<'info>,
    #[account(mut, has_one = borrower)]
    pub loan_account: Account<'info, Loan>,
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}
