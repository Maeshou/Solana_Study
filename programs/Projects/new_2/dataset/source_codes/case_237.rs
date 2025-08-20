use anchor_lang::prelude::*;

declare_id!("VulnEx36000000000000000000000000000000000036");

#[program]
pub mod example36 {
    pub fn repay_loan(ctx: Context<Ctx36>, amount: u64) -> Result<()> {
        // repay_history は所有者検証なし
        ctx.accounts.repay_history.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());
        // loan_account は has_one で borrower 検証済み
        let loan = &mut ctx.accounts.loan_account;
        loan.outstanding = loan.outstanding.saturating_sub(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx36<'info> {
    #[account(mut)]
    pub repay_history: AccountInfo<'info>,
    #[account(mut, has_one = borrower)]
    pub loan_account: Account<'info, LoanAccount>,
    pub borrower: Signer<'info>,
}

#[account]
pub struct LoanAccount {
    pub borrower: Pubkey,
    pub outstanding: u64,
}
