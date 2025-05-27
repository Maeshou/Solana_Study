use anchor_lang::prelude::*;

declare_id!("Prog05011111111111111111111111111111111");

#[program]
pub mod case050 {
    use super::*;

    pub fn borrow_liquidity(ctx: Context<Ctx050>) -> Result<()> {
        let a = &ctx.accounts.first;
        let b = &ctx.accounts.second;
        // 脆弱性: Duplicate Mutable Account のチェックをしていない
        msg!("Account A: {}", a.key());
        msg!("Account B: {}", b.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx050<'info> {
    #[account(mut)]
    pub first: AccountInfo<'info>,
    #[account(mut)]
    pub second: AccountInfo<'info>,
}
