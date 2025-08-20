use anchor_lang::prelude::*;
declare_id!("LOTT0231111111111111111111111111111111111111");

#[program]
pub mod case023 {
    use super::*;
    pub fn execute_lotterydraw(ctx: Context<LotteryDrawContext>) -> Result<()> {
        // Lottery or betting logic
        let mut game = GameAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        game.jackpot = game.jackpot.checked_add(50).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LotteryDrawContext<'info> {
    /// CHECK: expecting LotteryDrawAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting LotteryDrawAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LotteryDrawAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}