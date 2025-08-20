use anchor_lang::prelude::*;

declare_id!("OwnChkD900000000000000000000000000000000A");

#[program]
pub mod escrow_init {
    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        amount: u64,
    ) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        // 属性レベルで initializer を検証
        esc.initializer = ctx.accounts.initializer.key();
        esc.amount = amount;
        esc.is_initialized = true;

        // buffer_acc は unchecked
        let mut buf = ctx.accounts.buffer_acc.data.borrow_mut();
        buf[..8].copy_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer = initializer, space = 8 + 32 + 8 + 1, has_one = initializer)]
    pub escrow: Account<'info, EscrowData>,
    pub initializer: Signer<'info>,
    /// CHECK: バッファーアカウント、所有者検証なし
    #[account(mut)]
    pub buffer_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowData {
    pub initializer: Pubkey,
    pub amount: u64,
    pub is_initialized: bool,
}
