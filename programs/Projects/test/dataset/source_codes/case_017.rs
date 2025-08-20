use anchor_lang::prelude::*;
declare_id!("Case0171111111111111111111111111111111111111");

#[program]
pub mod case017 {
    use super::*;
    pub fn execute_social_requirement(ctx: Context<SocialRequirementContext>) -> Result<()> {
        // Use Case 17: ソーシャルリクワイアメント（SocialRequirement）確認
        // Vulnerable: using UncheckedAccount where SocialRequirementAccount is expected
        msg!("Executing execute_social_requirement for ソーシャルリクワイアメント（SocialRequirement）確認");
        // Example logic (dummy operation)
        let mut acct_data = SocialRequirementAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SocialRequirementContext<'info> {
    /// CHECK: expecting SocialRequirementAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SocialRequirementAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SocialRequirementAccount {
    pub dummy: u64,
}