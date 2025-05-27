use anchor_lang::prelude::*;

declare_id!("Prog07111111111111111111111111111111111");

#[program]
pub mod case071 {
    use super::*;

    pub fn init_metadata(ctx: Context<Ctx071>) -> Result<()> {
        let a = &ctx.accounts.first;
        let b = &ctx.accounts.second;
        // 脆弱性: Duplicate Mutable Account のチェックをしていない
        msg!("Account A: {}", a.key());
        msg!("Account B: {}", b.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx071<'info> {
    #[account(mut)]
    pub first: AccountInfo<'info>,
    #[account(mut)]
    pub second: AccountInfo<'info>,
}
