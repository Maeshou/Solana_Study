use anchor_lang::prelude::*;

declare_id!("Prog02111111111111111111111111111111111");

#[program]
pub mod case021 {
    use super::*;

    pub fn repay_flash_loan(ctx: Context<Ctx021>, amount: u64) -> Result<()> {
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
pub struct Ctx021<'info> {
    #[account(mut)]
    pub account_src: AccountInfo<'info>,
    #[account(mut)]
    pub account_dst: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
