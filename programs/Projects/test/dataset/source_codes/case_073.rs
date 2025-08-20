use anchor_lang::prelude::*;
declare_id!("Case0731111111111111111111111111111111111111");

#[program]
pub mod case073 {
    use super::*;
    pub fn execute_on_chain_render(ctx: Context<OnChainRenderContext>) -> Result<()> {
        // Use Case 73: オンチェーンレンダリング（OnChainRender）登録
        // Vulnerable: using UncheckedAccount where OnChainRenderAccount is expected
        msg!("Executing execute_on_chain_render for オンチェーンレンダリング（OnChainRender）登録");
        // Example logic (dummy operation)
        let mut acct_data = OnChainRenderAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OnChainRenderContext<'info> {
    /// CHECK: expecting OnChainRenderAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting OnChainRenderAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OnChainRenderAccount {
    pub dummy: u64,
}