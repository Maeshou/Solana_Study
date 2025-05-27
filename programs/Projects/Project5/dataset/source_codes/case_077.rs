use anchor_lang::prelude::*;

declare_id!("Prog07711111111111111111111111111111111");

#[program]
pub mod case077 {
    use super::*;

    pub fn take_loan(ctx: Context<Ctx077>) -> Result<()> {
        let a = &ctx.accounts.first;
        let b = &ctx.accounts.second;
        // 脆弱性: Duplicate Mutable Account のチェックをしていない
        msg!("Account A: {}", a.key());
        msg!("Account B: {}", b.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx077<'info> {
    #[account(mut)]
    pub first: AccountInfo<'info>,
    #[account(mut)]
    pub second: AccountInfo<'info>,
}
