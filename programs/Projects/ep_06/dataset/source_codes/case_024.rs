use anchor_lang::prelude::*;
declare_id!("PLAC0241111111111111111111111111111111111111");

#[program]
pub mod case024 {
    use super::*;
    pub fn execute_placebet(ctx: Context<PlaceBetContext>) -> Result<()> {
        // Lottery or betting logic
        let mut game = GameAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        game.jackpot = game.jackpot.checked_add(50).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBetContext<'info> {
    /// CHECK: expecting PlaceBetAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting PlaceBetAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlaceBetAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}