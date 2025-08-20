use anchor_lang::prelude::*;

declare_id!("VulnEscrow8888888888888888888888888888888888");

#[program]
pub mod vuln_escrow {
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let esc = &ctx.accounts.escrow;
        // esc.depositor 未検証で資金返却
        **ctx.accounts.depositor.lamports.borrow_mut() += esc.amount;
        **ctx.accounts.escrow_acc.lamports.borrow_mut() -= esc.amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub escrow: Account<'info, EscrowData>,
    /// CHECK: depositor 検証なし
    #[account(mut)]
    pub depositor: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_acc: AccountInfo<'info>,
}

#[account]
pub struct EscrowData {
    pub depositor: Pubkey,
    pub amount: u64,
}
