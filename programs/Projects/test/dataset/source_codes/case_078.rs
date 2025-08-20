use anchor_lang::prelude::*;
declare_id!("Case0781111111111111111111111111111111111111");

#[program]
pub mod case078 {
    use super::*;
    pub fn execute_digital_twin(ctx: Context<DigitalTwinContext>) -> Result<()> {
        // Use Case 78: デジタルツイン登録（DigitalTwin）
        // Vulnerable: using UncheckedAccount where DigitalTwinAccount is expected
        msg!("Executing execute_digital_twin for デジタルツイン登録（DigitalTwin）");
        // Example logic (dummy operation)
        let mut acct_data = DigitalTwinAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DigitalTwinContext<'info> {
    /// CHECK: expecting DigitalTwinAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DigitalTwinAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DigitalTwinAccount {
    pub dummy: u64,
}