// Scenario 034: 信用スコア（Credit Score）
use anchor_lang::prelude::*;
declare_id!("Fake11111111111111111111111111111111");

#[program]
pub mod case_034 {
    pub fn process(ctx: Context<Case_034>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.vault_account;
        let bal_before = account.to_account_info().lamports();
        **account.to_account_info().try_borrow_mut_lamports()? = bal_before.checked_sub(amount).unwrap_or(0);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case_034<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, Vault>,
    pub signer: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
