use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA27mvTWf");

#[program]
pub mod power_accumulator_003 {
    use super::*;

    pub fn contribute(ctx: Context<Ctx003>, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.storage;
        s.total_power += amount;
        s.last_contributed = amount;
        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Total Power: {}", s.total_power);
        msg!("Last Contribution: {}", s.last_contributed);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub total_power: u64,
    pub last_contributed: u64,
}
