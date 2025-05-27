use anchor_lang::prelude::*;

declare_id!("Prog01211111111111111111111111111111111");

#[program]
pub mod case012 {
    use super::*;

    pub fn create_vault(ctx: Context<Ctx012>, amount: u64) -> Result<()> {
        let src = &mut ctx.accounts.account_src;
        let dst = &mut ctx.accounts.account_dst;
        // 脆弱性: 重複ミュータブルアカウントチェックなし
        let before = **src.to_account_info().try_borrow_lamports()?;
        **src.to_account_info().try_borrow_mut_lamports()? = before.saturating_sub(amount);
        **dst.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx012<'info> {
    #[account(mut)]
    pub account_src: AccountInfo<'info>,
    #[account(mut)]
    pub account_dst: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
