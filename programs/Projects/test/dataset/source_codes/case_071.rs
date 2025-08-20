use anchor_lang::prelude::*;
declare_id!("Case0711111111111111111111111111111111111111");

#[program]
pub mod case071 {
    use super::*;
    pub fn execute_register_airdrop(ctx: Context<RegisterAirdropContext>) -> Result<()> {
        // Use Case 71: エアドロップ登録（RegisterAirdrop）
        // Vulnerable: using UncheckedAccount where RegisterAirdropAccount is expected
        msg!("Executing execute_register_airdrop for エアドロップ登録（RegisterAirdrop）");
        // Example logic (dummy operation)
        let mut acct_data = RegisterAirdropAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterAirdropContext<'info> {
    /// CHECK: expecting RegisterAirdropAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterAirdropAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterAirdropAccount {
    pub dummy: u64,
}