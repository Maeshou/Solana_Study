use anchor_lang::prelude::*;
declare_id!("Case0231111111111111111111111111111111111111");

#[program]
pub mod case023 {
    use super::*;
    pub fn execute_lottery_draw(ctx: Context<LotteryDrawContext>) -> Result<()> {
        // Use Case 23: ランダム抽選（LotteryDraw）
        // Vulnerable: using UncheckedAccount where LotteryDrawAccount is expected
        msg!("Executing execute_lottery_draw for ランダム抽選（LotteryDraw）");
        // Example logic (dummy operation)
        let mut acct_data = LotteryDrawAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}