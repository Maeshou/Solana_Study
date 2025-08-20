use anchor_lang::prelude::*;
declare_id!("GUIL0451111111111111111111111111111111111111");

#[program]
pub mod case045 {
    use super::*;
    pub fn execute_guildreward(ctx: Context<GuildRewardContext>) -> Result<()> {
        // Default context logic
        msg!("Case 45 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GuildRewardContext<'info> {
    /// CHECK: expecting GuildRewardAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting GuildRewardAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildRewardAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}