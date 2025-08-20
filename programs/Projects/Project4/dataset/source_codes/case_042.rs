use anchor_lang::prelude::*;

declare_id!("InitAll7777777777777777777777777777777777");

#[program]
pub mod multi_init7 {
    use super::*;

    // エスクロー、リリース履歴、手数料アカウントを初期化
    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        amount: u64,
    ) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        esc.initializer = ctx.accounts.initializer.key();
        esc.amount = amount;
        esc.active = true;

        let history = &mut ctx.accounts.release_history;
        history.entries = Vec::new();

        let fees = &mut ctx.accounts.fee_account;
        fees.collected = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer = initializer, space = 8 + 32 + 8 + 1)]
    pub escrow: Account<'info, EscrowData>,
    #[account(init, payer = initializer, space = 8 + 4 + (16 * 8))]
    pub release_history: Account<'info, ReleaseHistoryData>,
    #[account(init, payer = initializer, space = 8 + 8)]
    pub fee_account: Account<'info, FeeAccountData>,
    #[account(mut)] pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowData {
    pub initializer: Pubkey,
    pub amount: u64,
    pub active: bool,
}

#[account]
pub struct ReleaseHistoryData {
    pub entries: Vec<(i64, u64)>, // (timestamp, released_amount)
}

#[account]
pub struct FeeAccountData {
    pub collected: u64,
}
