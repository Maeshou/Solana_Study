use anchor_lang::prelude::*;

declare_id!("Pay11111111111111111111111111111111");

#[program]
pub mod misinit_payment {
    use super::*;

    pub fn init_payment(
        ctx: Context<InitPayment>,
        amount: u64,
    ) -> Result<()> {
        let p = &mut ctx.accounts.payment;
        p.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPayment<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub payment: Account<'info, PaymentData>,

    /// 本来 init すべき FeeData が mut のみ
    #[account(mut)]
    pub fee: Account<'info, FeeData>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PaymentData {
    pub amount: u64,
}

#[account]
pub struct FeeData {
    pub collected: u64,
}
