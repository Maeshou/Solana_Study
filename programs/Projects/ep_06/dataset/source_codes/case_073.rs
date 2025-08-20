use anchor_lang::prelude::*;
declare_id!("ONCH0731111111111111111111111111111111111111");

#[program]
pub mod case073 {
    use super::*;
    pub fn execute_onchainrenderregister(ctx: Context<OnChainRenderRegisterContext>) -> Result<()> {
        // Content logic
        msg!("Content transaction executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OnChainRenderRegisterContext<'info> {
    /// CHECK: expecting OnChainRenderRegisterAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting OnChainRenderRegisterAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OnChainRenderRegisterAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}