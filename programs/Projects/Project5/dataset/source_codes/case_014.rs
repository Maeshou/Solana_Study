use anchor_lang::prelude::*;

declare_id!("Prog01411111111111111111111111111111111");

#[program]
pub mod case014 {
    use super::*;

    pub fn withdraw_from_vault(ctx: Context<Ctx014>) -> Result<()> {
        let a = &ctx.accounts.first;
        let b = &ctx.accounts.second;
        // 脆弱性: Duplicate Mutable Account のチェックをしていない
        msg!("Account A: {}", a.key());
        msg!("Account B: {}", b.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx014<'info> {
    #[account(mut)]
    pub first: AccountInfo<'info>,
    #[account(mut)]
    pub second: AccountInfo<'info>,
}
