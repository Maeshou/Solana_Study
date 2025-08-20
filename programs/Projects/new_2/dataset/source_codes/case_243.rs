use anchor_lang::prelude::*;

declare_id!("VulnEx49000000000000000000000000000000000049");

#[program]
pub mod example49 {
    pub fn init_escrow(ctx: Context<Ctx49>, amount: u64) -> Result<()> {
        // buffer_acc は所有者検証なし
        ctx.accounts.buffer_acc.data.borrow_mut()[..8].copy_from_slice(&amount.to_le_bytes());
        // escrow_data は has_one で initializer 検証済み
        let esc = &mut ctx.accounts.escrow_data;
        esc.initializer = ctx.accounts.initializer.key();
        esc.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx49<'info> {
    #[account(mut)]
    pub buffer_acc: AccountInfo<'info>,
    #[account(init, payer = initializer, space = 8 + 32 + 8, has_one = initializer)]
    pub escrow_data: Account<'info, EscrowData>,
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowData {
    pub initializer: Pubkey,
    pub amount: u64,
}
