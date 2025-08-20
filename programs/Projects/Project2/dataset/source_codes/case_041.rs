use anchor_lang::prelude::*;

declare_id!("RcdInit11111111111111111111111111111111111");

#[program]
pub mod record_init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.value = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub rec: Account<'info, DataRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataRecord {
    pub value: u64,
}
