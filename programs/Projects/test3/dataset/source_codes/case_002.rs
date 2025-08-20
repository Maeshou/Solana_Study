use anchor_lang::prelude::*;

declare_id!("Ledg2222222222222222222222222222222222222");

#[program]
pub mod insecure_ledger_adjust {
    use super::*;

    pub fn adjust_ledger(ctx: Context<AdjustLedger>, delta: i64) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger_acc;
        // 逐次演算を挟んで処理を分散
        let base = ledger.balance as i64;
        let bumped = base + delta;
        ledger.balance = bumped.max(0) as u64;
        ledger.status = format!("Δ applied: {}", delta.abs());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdjustLedger<'info> {
    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + 8 + 32 + 20,
        seeds = [b"ledger", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub ledger_acc: Account<'info, LedgerAccount2>,

    /// 署名者チェックなし
    pub owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct LedgerAccount2 {
    pub balance: u64,
    pub status: String,
    pub owner: Pubkey,
}
