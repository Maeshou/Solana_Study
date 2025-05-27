use anchor_lang::prelude::*;

declare_id!("Prog00811111111111111111111111111111111");

#[program]
pub mod case008 {
    use super::*;

    pub fn cancel_escrow(ctx: Context<Ctx008>) -> Result<()> {
        let a = &ctx.accounts.first;
        let b = &ctx.accounts.second;
        // 脆弱性: Duplicate Mutable Account のチェックをしていない
        msg!("Account A: {}", a.key());
        msg!("Account B: {}", b.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx008<'info> {
    #[account(mut)]
    pub first: AccountInfo<'info>,
    #[account(mut)]
    pub second: AccountInfo<'info>,
}
